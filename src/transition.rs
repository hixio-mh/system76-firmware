static TRANSITIONS: &'static [Transition] = &[
    Transition::new("addw2", "PBx0Dx2", false),
    Transition::new("darp6", "N150ZU", false),
    Transition::new("darp6", "N150CU", false), // TODO: set liberate to true when tested
    Transition::new("galp3-c", "N130ZU", false),
    Transition::new("galp4", "N140CU", false), // TODO: set liberate to true when tested
    Transition::new("gaze15", "NH5xDC", false), //TODO: Support variant detection for NH50DB
];

struct Transition {
    /// Model name
    model: &'static str,
    /// Open EC project, always "76ec"
    open: &'static str,
    /// Proprietary EC project
    proprietary: &'static str,
    /// If true, TransitionKind::Automatic will switch to open firmware
    liberate: bool,
}

impl Transition {
    const fn new(model: &'static str, proprietary: &'static str, liberate: bool) -> Self {
        Self {
            model,
            open: "76ec",
            proprietary,
            liberate
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TransitionKind {
    /// Whatever the default is
    Automatic,
    /// Open firmware, if available
    Open,
    /// Proprietary firmware, if available
    Proprietary,
}

impl TransitionKind {
    pub fn transition(self, model: &str, project: &str) -> (String, String) {
        for transition in TRANSITIONS.iter() {
            if model == transition.model {
                let new_project = if project == transition.open {
                    match self {
                        TransitionKind::Automatic => transition.open,
                        TransitionKind::Open => transition.open,
                        TransitionKind::Proprietary => transition.proprietary,
                    }
                } else if project == transition.proprietary {
                    match self {
                        TransitionKind::Automatic => if transition.liberate {
                            transition.open
                        } else {
                            transition.proprietary
                        },
                        TransitionKind::Open => transition.open,
                        TransitionKind::Proprietary => transition.proprietary,
                    }
                } else {
                    project
                };

                return (model.to_string(), new_project.to_string());
            }
        }

        (model.to_string(), project.to_string())
    }
}
