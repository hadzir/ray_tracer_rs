use serde::Deserialize;

use crate::body::VBody;
use crate::camera::*;
use crate::light::VPointLight;

#[derive(Debug, Deserialize)]
struct WorldConfig {
    cam: VCamera,
    light: VPointLight,
    body: VBody,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::VMaterial;
    use crate::material::VPhong;
    use crate::matrix::VMatrix;
    use crate::sphere::VSphere;
    use crate::tuple::VTuple;
    use crate::world::*;
    use std::collections::HashMap;
    use crate::canvas::vcolor::VColor;
    use ron::de::from_str;

    //use crate::zequality::*;
    #[test]
    fn load_yaml_world() {
        let expected_world = VWorld::new(
            vec![VBody::from(
                VSphere::default()
                    .with_transform(VMatrix::rotation_x(3.14) * VMatrix::translation(1.0, 2.0, 3.0))
                    .with_material(VMaterial::from(VPhong::new(
                        VColor::new(1.0, 1.0, 1.0),
                        0.1,
                        0.7,
                        0.0,
                        200.0,
                    ))),
            )],
            vec![VPointLight::new(
                VTuple::point(1.1, 2.2, 3.3),
                VColor::new(0.4, 0.5, 0.6),
            )],
        );

        let mut expected_cameras = HashMap::new();
        expected_cameras.insert(
            String::from("output1"),
            VCamera::new(800, 600, 0.785).positioned_and_pointed(
                VTuple::point(1.0, 2.0, 3.4),
                VTuple::point(5.6, 7.0, 8.0),
                VTuple::vector(9.10, 11.0, 1.2),
            ),
        );
        let cfg: WorldConfig = match from_str("worldconfig.ron") {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);

                std::process::exit(1);
            }
        };

        // println!("Camera: {:?}", &cfg.cam);
        // println!("Light: {:?}", &light);
        println!("Body: {:?}", &cfg.body);

        //assert_zeq!(loaded_world,expected_world);
        //assert_zeq!(loaded_cameras,expected_cameras);
    }
}
