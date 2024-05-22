use raylib::{models::{Model, RaylibMaterial, RaylibModel}, RaylibHandle, RaylibThread, texture::Texture2D};

macro_rules! assets {
    () => {};
    ($($name:ident: $struct_name:ident($type:tt => $load_fn:tt) {$($subname:ident: $path:literal),*}),*) => {
        $(
            pub struct $struct_name {
                $(
                    pub $subname: $type,
                )*
            }
        )*

        pub struct Assets {
            $(
                pub $name: $struct_name
            ),*
        }

        impl Assets {
            pub fn load(rl: &mut raylib::RaylibHandle, thread: &raylib::RaylibThread) -> anyhow::Result<Self> {
                let mut s = Self {
                    $(
                        $name: $struct_name {
                            $(
                                $subname: $load_fn(rl, thread, concat!("./assets/", $path)).map_err(|e| anyhow::anyhow!(e))?,
                            )*
                        },
                    )*
                };

                assets_finish(rl, thread, &mut s);

                Ok(s)
            }
        }
    };
}

fn load_model(rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Result<Model, String> {
    rl.load_model(thread, path)
}

fn load_texture(rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Result<Texture2D, String> {
    rl.load_texture(thread, path)
}

assets! {
    models: Models(Model => load_model) { neptune: "models/neptune.obj" },
    textures: Textures(Texture2D => load_texture) { neptune_surface: "textures/2k_neptune.png" }
}

fn assets_finish(rl: &mut RaylibHandle, thread: &RaylibThread, assets: &mut Assets) {
    assets.models.neptune.materials_mut()[0]
        .set_material_texture(raylib::ffi::MaterialMapIndex::MATERIAL_MAP_ALBEDO, &assets.textures.neptune_surface);
}