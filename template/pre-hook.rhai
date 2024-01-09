let project_name = variable::get("project-name");

file::rename("project_name", `${project_name}`);
file::rename("project_name_macros", `${project_name}_macros`);
file::rename("project_name_types", `${project_name}_types`);
