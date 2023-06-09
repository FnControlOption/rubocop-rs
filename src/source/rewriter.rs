pub struct Rewriter {
    action_root: Action,
}

#[derive(Clone, Default)]
struct Action {
    begin_pos: usize,
    end_pos: usize,
    replacement: Option<String>,
    insert_before: String,
    insert_after: String,
    children: Vec<Action>,
}

struct Family {
    parent: Option<Action>,
    sibling_left: Vec<Action>,
    sibling_right: Vec<Action>,
    fusible: Option<Vec<Action>>,
    child: Option<Vec<Action>>,
}

impl Rewriter {
    pub fn new(code: &[u8]) -> Rewriter {
        Rewriter {
            action_root: Action {
                begin_pos: 0,
                end_pos: code.len(),
                ..Default::default()
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        self.action_root.is_empty()
    }

    pub fn replace(&mut self, begin_pos: usize, end_pos: usize, content: String) {
        self.combine(Action {
            begin_pos,
            end_pos,
            replacement: Some(content),
            ..Default::default()
        });
    }

    pub fn wrap(
        &mut self,
        begin_pos: usize,
        end_pos: usize,
        insert_before: String,
        insert_after: String,
    ) {
        self.combine(Action {
            begin_pos,
            end_pos,
            insert_before,
            insert_after,
            ..Default::default()
        });
    }

    pub fn remove(&mut self, begin_pos: usize, end_pos: usize) {
        self.replace(begin_pos, end_pos, "".to_string());
    }

    pub fn insert_before(&mut self, pos: usize, content: String) {
        self.wrap(pos, pos, content, "".to_string());
    }

    pub fn insert_after(&mut self, pos: usize, content: String) {
        self.wrap(pos, pos, "".to_string(), content);
    }

    pub fn process(self, code: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        let mut last_end = 0;
        for (begin_pos, end_pos, replacement) in self.action_root.ordered_replacements() {
            result.extend(&code[last_end..begin_pos]);
            result.extend(replacement.bytes());
            last_end = end_pos;
        }
        result.extend(&code[last_end..code.len()]);
        result
    }

    fn combine(&mut self, action: Action) {
        self.action_root.combine(action);
    }
}

impl Action {
    fn combine(&mut self, action: Action) {
        if action.is_empty() {
            return;
        }

        if (action.begin_pos, action.end_pos) == (self.begin_pos, self.end_pos) {
            self.merge(action)
        } else {
            self.place_in_hierarchy(action)
        }
    }

    fn is_empty(&self) -> bool {
        let replacement = self.replacement.as_ref();

        self.insert_before.is_empty()
            && self.insert_after.is_empty()
            && self.children.is_empty()
            && replacement.map_or(true, |r| r.is_empty() && self.begin_pos == self.end_pos)
    }

    fn ordered_replacements(self) -> Vec<(usize, usize, String)> {
        let Action {
            begin_pos,
            end_pos,
            replacement,
            insert_before,
            insert_after,
            children,
        } = self;

        let mut reps = Vec::new();
        if !insert_before.is_empty() {
            reps.push((begin_pos, begin_pos, insert_before));
        }
        if let Some(replacement) = replacement {
            reps.push((begin_pos, end_pos, replacement));
        }
        for child in children {
            reps.append(&mut child.ordered_replacements());
        }
        if !insert_after.is_empty() {
            reps.push((begin_pos, begin_pos, insert_after));
        }
        reps
    }

    #[allow(dead_code)]
    fn has_insertion(&self) -> bool {
        let replacement = self.replacement.as_ref();

        !self.insert_before.is_empty()
            || !self.insert_after.is_empty()
            || replacement.map_or(false, |r| !r.is_empty())
    }

    fn place_in_hierarchy(&mut self, mut action: Action) {
        let family = self.analyze_hierarchy(&action);
        let mut siblings = family.sibling_left;
        let mut sibling_right = family.sibling_right;

        if let Some(fusible) = family.fusible {
            if let Some(mut child) = family.child {
                siblings.append(&mut child);
            }
            siblings.append(&mut sibling_right);
            self.fuse_deletions(action, fusible, siblings)
        } else {
            let extra_sibling = if let Some(mut parent) = family.parent {
                parent.combine(action);
                parent
            } else if let Some(child) = family.child {
                let tmp = action.children;
                action.children = child;
                action.combine_children(tmp);
                action
            } else {
                action
            };
            siblings.push(extra_sibling);
            siblings.append(&mut sibling_right);
            self.children = siblings;
        }
    }

    fn combine_children(&mut self, more_children: Vec<Action>) {
        for new_child in more_children {
            self.place_in_hierarchy(new_child);
        }
    }

    fn fuse_deletions(
        &mut self,
        action: Action,
        mut fusible: Vec<Action>,
        other_siblings: Vec<Action>,
    ) {
        self.children = other_siblings;

        fusible.insert(0, action.clone());

        let mut fused_deletion = action;
        fused_deletion.begin_pos = fusible.iter().map(|a| a.begin_pos).min().unwrap();
        fused_deletion.end_pos = fusible.iter().map(|a| a.end_pos).max().unwrap();

        self.combine(fused_deletion)
    }

    fn bsearch_child_index<F>(&self, from: usize, f: F) -> usize
    where
        F: Fn(&Action) -> bool,
    {
        let size = self.children.len();
        (from..size).find(|&i| f(&self.children[i])).unwrap_or(size)
    }

    // TODO: investigate if Vec::split_off can be used here
    fn analyze_hierarchy(&self, action: &Action) -> Family {
        let mut parent: Option<Action> = None;
        let mut fusible: Option<Vec<Action>> = None;
        let mut contained: Option<Vec<Action>> = None;

        let mut left_index = self.bsearch_child_index(0, |child| child.end_pos > action.begin_pos);
        let start = if left_index == 0 { 0 } else { left_index - 1 };
        let mut right_index =
            self.bsearch_child_index(start, |child| child.begin_pos >= action.end_pos);
        // TODO: Would overflowing_sub work?
        let center = right_index as i128 - left_index as i128;

        match center {
            0 => {}
            -1 => {
                left_index -= 1;
                right_index += 1;
                parent = Some(self.children[left_index].clone());
            }
            _ => {
                let overlap_left = self.children[left_index].begin_pos.cmp(&action.begin_pos);
                let overlap_right = self.children[right_index - 1].end_pos.cmp(&action.end_pos);

                if center == 1 && overlap_left.is_le() && overlap_right.is_ge() {
                    parent = Some(self.children[left_index].clone());
                } else {
                    (contained, fusible) = {
                        let mut contained = self.children[left_index..right_index].to_vec();
                        let mut fusible = Vec::new();
                        if overlap_left.is_lt() {
                            fusible.push(contained.remove(0));
                        }
                        if overlap_right.is_gt() {
                            fusible.push(contained.pop().unwrap());
                        }
                        if fusible.is_empty() {
                            (Some(contained), None)
                        } else {
                            (Some(contained), Some(fusible))
                        }
                    };
                }
            }
        }

        Family {
            parent,
            sibling_left: self.children[0..left_index].to_vec(),
            sibling_right: self.children[right_index..self.children.len()].to_vec(),
            fusible,
            child: contained,
        }
    }

    fn merge(&mut self, action: Action) {
        if action.replacement.is_some() {
            self.replacement = action.replacement;
        }
        if self.replacement.is_some() {
            self.children = Vec::new();
        }
        self.insert_before.insert_str(0, &action.insert_before);
        self.insert_after.push_str(&action.insert_after);
        self.combine_children(action.children)
    }
}
