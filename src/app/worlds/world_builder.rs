use std::{ops::Range, rc::Rc};


use ray_tracing_in_a_weekend::{*, rect_geometry::{RectGeometry, RectPlane}};

pub struct WorldBuilder<'a> {
    pub arena: &'a mut bumpalo::Bump
}

impl<'a> WorldBuilder<'a> {
    pub fn new(arena: &'a mut bumpalo::Bump) -> Self { Self { arena }}

    pub fn texture_solid(&self, color: Color) -> &Texture {
        self.alloc(Texture::Solid { color })
    }
    pub fn texture_image_from_file(&self, path : &std::path::Path, fmt : image::ImageFormat) -> &Texture {
        let file = std::fs::OpenOptions::new().read(true).open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let image = self.alloc(image::load(reader, fmt).unwrap());
    
        self.alloc(Texture::Image {
            image: image.as_rgb8().unwrap(),
        })
    }
    pub fn material_diffuse_light_solid(&self, color: Color) -> &Material {
        let emit = self.texture_solid(color);
        self.alloc(Material::DiffuseLight { emit })
    }
    pub fn material_lambert_solid(&self, color: Color) -> &Material {
        let albedo = self.texture_solid(color);
        self.material_lambert(albedo)
    }
    pub fn material_lambert(&'a self, albedo : &'a Texture) -> &'a Material {
        self.alloc(Material::Lambert { albedo })
    }
    
    pub fn material_metal_solid(&self, color: Color, fuzz: f32) -> &Material {
        let albedo = self.texture_solid(color);
        self.alloc(Material::Metal { albedo, fuzz })
    }
    pub fn material_dielectric(&self, index_of_refraction: f32) -> &Material {
        self.alloc(Material::Dielectric { index_of_refraction })
    }

    fn geo_rect_xy(&self, center : Point3, s0 : f32, s1 : f32) -> Geometry { self.geo_rect(RectPlane::Xy, center, s0, s1) }
    fn geo_rect_yz(&self, center : Point3, s0 : f32, s1 : f32) -> Geometry { self.geo_rect(RectPlane::Yz, center, s0, s1) }
    fn geo_rect_xz(&self, center : Point3, s0 : f32, s1 : f32) -> Geometry { self.geo_rect(RectPlane::Xz, center, s0, s1) }
    fn geo_rect(&self, rect_plane : RectPlane, center : Point3, s0 : f32, s1 : f32) -> Geometry {
        let (a0, a1, n) = rect_plane.get_axis();
        let geo = RectGeometry {
            rect_plane,
            dist: center.0.e[n],
            r0: ((center.0.e[a0] - s0 * 0.5), (center.0.e[a0] + s0 * 0.5)),
            r1: ((center.0.e[a1] - s1 * 0.5), (center.0.e[a1] + s1 * 0.5)),
        };
        Geometry::Rect(geo)
    }
    
    fn geo_box(&self, width: f32, height: f32, depth: f32) -> Geometry {
        Geometry::AxisAlignedBox(Aabb::new_corners(Point3::ORIGIN, Point3::new(width, height, depth)))
    }
    fn geo_sphere(&self, radius : f32) -> Geometry {
        Geometry::Sphere(sphere_geometry::SphereGeometry{center: Point3::ORIGIN, radius})
    }

    pub fn alloc<T>(&self, v : T) -> &mut T {
        self.arena.alloc(v)
    }

    pub fn new_group(&self) -> GroupRef {
        GroupRef { elements: Vec::new() }
    }
    pub fn new_obj(&self, geometry : Geometry) -> ObjectRef {
        ObjectRef { geometry, material: None, transformation:Transformation::ZERO }
    }

    pub fn new_obj_rect_yz(&self, position: Point3, size0 : f32, size1: f32) -> ObjectRef {
        self.new_obj(self.geo_rect_yz(position, size0, size1))
    }

    pub fn new_obj_rect_xz(&self, position: Point3, size0: f32, size1: f32) -> ObjectRef {
        self.new_obj(self.geo_rect_xz(position, size0, size1))
    }

    pub fn new_obj_rect_xy(&self, position: Point3, size0: f32, size1: f32) -> ObjectRef {
        self.new_obj(self.geo_rect_xy(position, size0, size1))
    }

    pub fn new_obj_box(&self, width: f32, height: f32, depth: f32) -> ObjectRef {
        self.new_obj(self.geo_box(width, height, depth))
    }
    pub fn new_obj_sphere(&self, radius : f32) -> ObjectRef {
        self.new_obj(self.geo_sphere(radius))
    }

    pub fn group_to_hittable(&'a self, group: GroupRef<'a>, _time_range : &Range<f32>) -> &'a Scene<'a> {
        let mut elements = Vec::<&'a SceneElement<'a>>::new();

        for elem in group.elements {
            let (real_geo, remaining_transformation) = elem.geometry.partial_apply_transformation(&elem.transformation);
            let geo_elem = self.alloc(SceneElement::Geometry(real_geo, elem.material.unwrap()));
            let elem = if let Some(trans) = remaining_transformation {
                self.alloc(SceneElement::Transformation(geo_elem, trans))
            } else {
                geo_elem
            };
            elements.push(elem);
        }
        let root = self.alloc(SceneElement::Group(elements));
        self.alloc(Scene::new(root))
    }
}

enum NodeData<'a> {
    Object(&'a Geometry, Option<&'a Material<'a>>),
    Group(Vec<NodeRef<'a>>),
}
pub struct Node<'a> {
    data : NodeData<'a>,
    transformation : Transformation,
}
pub struct NodeRef<'a>(Rc<Node<'a>>); 

pub struct GroupRef<'a>{
    elements : Vec<ObjectRef<'a>>,
}

impl<'a> GroupRef<'a> {
    pub fn add(&mut self, elem : ObjectRef<'a>) {
        self.elements.push(elem);
    }
}

#[derive(Clone, Copy)]
pub struct ObjectRef<'a> {
    geometry : Geometry,
    material : Option<&'a Material<'a>>,
    transformation : Transformation,
}

impl<'a> ObjectRef<'a> {
    pub fn set_material(mut self, material: &'a Material) -> ObjectRef<'a> {
        self.material = Some(material);
        self
    }

    pub fn set_transform(mut self, transformation: Transformation) -> ObjectRef<'a> {
        self.transformation = transformation;
        self
    }
    pub fn rotate_around_up(mut self, angle : f32) -> ObjectRef<'a> {
        self.transformation = self.transformation.rotate_around_up(angle);
        self
    }
    pub fn translate(mut self, offset : Dir3) -> ObjectRef<'a> {
        self.transformation = self.transformation.translate(offset);
        self
    }
    pub fn get_world_scattering_provider(&self) -> Option<WorldScatteringDistributionProvider> {
        self.geometry.get_world_scattering_provider()
    }
}
