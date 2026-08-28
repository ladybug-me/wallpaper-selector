use crate::domain::theme::ThemeRole;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoleDescriptor {
    pub role: ThemeRole,
    pub name: &'static str,
    pub description: &'static str,
}

pub const ROLES: [RoleDescriptor; 9] = [
    RoleDescriptor {
        role: ThemeRole::Primary,
        name: "theme-designer-role-accent",
        description: "theme-designer-role-accent-desc",
    },
    RoleDescriptor {
        role: ThemeRole::PrimaryText,
        name: "theme-designer-role-accent-text",
        description: "theme-designer-role-accent-text-desc",
    },
    RoleDescriptor {
        role: ThemeRole::Tertiary,
        name: "theme-designer-role-second-accent",
        description: "theme-designer-role-second-accent-desc",
    },
    RoleDescriptor {
        role: ThemeRole::Surface,
        name: "theme-designer-role-panel",
        description: "theme-designer-role-panel-desc",
    },
    RoleDescriptor {
        role: ThemeRole::SurfaceText,
        name: "theme-designer-role-panel-text",
        description: "theme-designer-role-panel-text-desc",
    },
    RoleDescriptor {
        role: ThemeRole::SurfaceVariant,
        name: "theme-designer-role-panel-alt",
        description: "theme-designer-role-panel-alt-desc",
    },
    RoleDescriptor {
        role: ThemeRole::SurfaceContainer,
        name: "theme-designer-role-panel-raised",
        description: "theme-designer-role-panel-raised-desc",
    },
    RoleDescriptor {
        role: ThemeRole::Background,
        name: "theme-designer-role-backdrop",
        description: "theme-designer-role-backdrop-desc",
    },
    RoleDescriptor {
        role: ThemeRole::Outline,
        name: "theme-designer-role-outline",
        description: "theme-designer-role-outline-desc",
    },
];
