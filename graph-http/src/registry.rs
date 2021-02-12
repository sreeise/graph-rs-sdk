use graph_core::resource::ResourceIdentity;
use handlebars::*;

#[derive(Default)]
pub struct Registry;

impl Registry {
    pub fn register_internal_helper(
        resource_identity: ResourceIdentity,
        registry: &mut Handlebars,
    ) {
        match resource_identity {
            ResourceIdentity::Drive |
            ResourceIdentity::Drives |
            ResourceIdentity::Me |
            ResourceIdentity::Users |
            ResourceIdentity::Sites |
            ResourceIdentity::Groups => {
                registry.register_helper(
                    "drive_item_id",
                    Box::new(
                        move |_: &Helper,
                              _: &Handlebars,
                              context: &Context,
                              _: &mut RenderContext,
                              out: &mut dyn Output|
                              -> HelperResult {
                            if let Some(data) = context.data().as_object() {
                                for (name, value) in data.iter() {
                                    if name.eq("id") {
                                        if let Some(id_str) = value.as_str() {
                                            match resource_identity {
                                                ResourceIdentity::Me |
                                                ResourceIdentity::Users |
                                                ResourceIdentity::Sites |
                                                ResourceIdentity::Groups => {
                                                    out.write("drive/")?;
                                                },
                                                _ => {},
                                            }

                                            if id_str.starts_with(':') && id_str.ends_with(":") {
                                                out.write("root")?;
                                            } else {
                                                out.write("items/")?;
                                            }
                                            out.write(id_str)?;
                                        }
                                    }
                                }
                            }

                            Ok(())
                        },
                    ),
                );

                registry.register_helper(
                    "resource_drive_path",
                    Box::new(
                        move |_: &Helper,
                              _: &Handlebars,
                              _: &Context,
                              _: &mut RenderContext,
                              out: &mut dyn Output|
                              -> HelperResult {
                            match resource_identity {
                                ResourceIdentity::Me |
                                ResourceIdentity::Users |
                                ResourceIdentity::Sites |
                                ResourceIdentity::Groups => {
                                    out.write("drive")?;
                                },
                                _ => {},
                            }
                            Ok(())
                        },
                    ),
                );
            },
            _ => {},
        }
    }
}
