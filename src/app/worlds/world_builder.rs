use ray_tracing_in_a_weekend::{
    rect_geometry::{RectGeometry, RectPlane},
    *,
};
use std::{ops::Range, rc::Rc};

pub struct WorldBuilder<'a> {
    pub arena: &'a mut bumpalo::Bump,
}

impl<'a> WorldBuilder<'a> {
    pub fn new(arena: &'a mut bumpalo::Bump) -> Self {
        Self { arena }
    }

    pub fn texture_solid(&self, color: Color) -> &Texture {
        self.alloc(Texture::Solid { color })
    }
    pub fn texture_image_from_file(
        &self,
        path: &std::path::Path,
        fmt: image::ImageFormat,
    ) -> &Texture {
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
    pub fn material_lambert(&'a self, albedo: &'a Texture) -> &'a Material {
        self.alloc(Material::Lambert { albedo })
    }

    pub fn material_metal_solid(&self, color: Color, fuzz: f32) -> &Material {
        let albedo = self.texture_solid(color);
        self.alloc(Material::Metal { albedo, fuzz })
    }
    pub fn material_dielectric(&self, index_of_refraction: f32) -> &Material {
        self.alloc(Material::Dielectric {
            index_of_refraction,
        })
    }

    fn geo_rect_xy(&self, center: Point3, s0: f32, s1: f32) -> Geometry {
        self.geo_rect(RectPlane::Xy, center, s0, s1)
    }
    fn geo_rect_yz(&self, center: Point3, s0: f32, s1: f32) -> Geometry {
        self.geo_rect(RectPlane::Yz, center, s0, s1)
    }
    fn geo_rect_xz(&self, center: Point3, s0: f32, s1: f32) -> Geometry {
        self.geo_rect(RectPlane::Xz, center, s0, s1)
    }
    fn geo_rect(&self, rect_plane: RectPlane, center: Point3, s0: f32, s1: f32) -> Geometry {
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
        Geometry::AxisAlignedBox(Aabb::new_corners(
            Point3::ORIGIN,
            Point3::new(width, height, depth),
        ))
    }
    fn geo_sphere(&self, radius: f32) -> Geometry {
        Geometry::Sphere(sphere_geometry::SphereGeometry {
            center: Point3::ORIGIN,
            radius,
        })
    }

    pub fn alloc<T>(&self, v: T) -> &mut T {
        self.arena.alloc(v)
    }

    pub fn new_group(&self) -> NodeBuilder {
        NodeBuilder(Box::new(Node {
            geo: Vec::new(),
            transformation: Transformation::ZERO,
            moving_animation: Dir3::ZERO,
            children: Vec::new(),
        }))
    }
    pub fn new_obj(&self, geometry: Geometry, material: &'a Material<'a>) -> NodeBuilder {
        NodeBuilder(Box::new(Node {
            geo: vec![(geometry, material, false)],
            transformation: Transformation::ZERO,
            moving_animation: Dir3::ZERO,
            children: Vec::new(),
        }))
    }

    pub fn new_obj_rect_yz(
        &self,
        position: Point3,
        size0: f32,
        size1: f32,
        material: &'a Material,
    ) -> NodeBuilder {
        self.new_obj(self.geo_rect_yz(position, size0, size1), material)
    }

    pub fn new_obj_rect_xz(
        &self,
        position: Point3,
        size0: f32,
        size1: f32,
        material: &'a Material,
    ) -> NodeBuilder {
        self.new_obj(self.geo_rect_xz(position, size0, size1), material)
    }

    pub fn new_obj_rect_xy(
        &self,
        position: Point3,
        size0: f32,
        size1: f32,
        material: &'a Material,
    ) -> NodeBuilder {
        self.new_obj(self.geo_rect_xy(position, size0, size1), material)
    }

    pub fn new_obj_box(
        &self,
        width: f32,
        height: f32,
        depth: f32,
        material: &'a Material,
    ) -> NodeBuilder {
        self.new_obj(self.geo_box(width, height, depth), material)
    }
    pub fn new_obj_sphere(&self, radius: f32, material: &'a Material) -> NodeBuilder {
        self.new_obj(self.geo_sphere(radius), material)
    }
    pub fn new_mesh_from_file_obj_uniform_material(
        &self,
        path: &std::path::Path,
        material: &'a Material<'a>,
    ) -> NodeBuilder {
        let file = std::fs::OpenOptions::new().read(true).open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let tris = crate::obj_loader::load_obj_mesh(reader).unwrap();
        let geo = tris
            .into_iter()
            .map(|t| (Geometry::Triangle(t), material, false))
            .collect::<Vec<_>>();
        NodeBuilder(Box::new(Node {
            geo,
            transformation: Transformation::ZERO,
            moving_animation: Dir3::ZERO,
            children: Vec::new(),
        }))
    }
}

struct Node<'a> {
    geo: Vec<(Geometry, &'a Material<'a>, bool)>,
    transformation: Transformation,
    moving_animation: Dir3,
    children: Vec<NodeRef<'a>>,
}

pub struct NodeBuilder<'a>(Box<Node<'a>>);

#[derive(Clone)]
pub struct NodeRef<'a>(Rc<Node<'a>>);

pub trait NodeLike<'a> {
    fn get(self) -> NodeRef<'a>;
}

impl<'a> NodeLike<'a> for NodeBuilder<'a> {
    fn get(self) -> NodeRef<'a> {
        self.build()
    }
}

impl<'a> NodeLike<'a> for &NodeRef<'a> {
    fn get(self) -> NodeRef<'a> {
        self.clone()
    }
}

impl<'a> NodeBuilder<'a> {
    pub fn add<T: NodeLike<'a>>(mut self, elem: T) -> Self {
        self.0.children.push(elem.get());
        self
    }

    pub fn set_all_geo_as_poi(mut self) -> Self {
        for geo in &mut self.0.geo {
            geo.2 = true;
        }
        self
    }

    pub fn set_transform(mut self, transformation: Transformation) -> Self {
        self.0.transformation = transformation;
        self
    }
    pub fn rotate_around_up(mut self, angle: f32) -> Self {
        self.0.transformation = self.0.transformation.rotate_around_up(angle);
        self
    }
    pub fn translate(mut self, offset: Dir3) -> Self {
        self.0.transformation = self.0.transformation.translate(offset);
        self
    }
    pub fn animate_moving(mut self, velocity: Dir3) -> Self {
        self.0.moving_animation += velocity;
        self
    }
    pub fn build(self) -> NodeRef<'a> {
        NodeRef(Rc::from(self.0))
    }
}

impl<'a> NodeRef<'a> {
    pub fn finish(
        self,
        wb: &'a WorldBuilder<'a>,
        time_range: &Range<f32>,
    ) -> (&'a Scene<'a>, Option<WorldScatteringDistributionProvider>) {
        let (root, wsd) = self.finish_internal(wb, time_range, &Transformation::ZERO);
        (wb.alloc(Scene::new(root)), wsd)
    }
    fn finish_internal(
        &self,
        wb: &'a WorldBuilder<'a>,
        time_range: &Range<f32>,
        parent_transform: &Transformation,
    ) -> (
        &'a SceneElement<'a>,
        Option<WorldScatteringDistributionProvider>,
    ) {
        let mut elements = Vec::<&'a SceneElement<'a>>::new();

        let full_trans = parent_transform.then(&self.0.transformation);
        let mut wsd = None;
        for (geo, material, is_poi) in &self.0.geo {
            let (real_geo, remaining_transformation) =
                geo.partial_apply_transformation(&full_trans);
            let mut elem = wb.alloc(SceneElement::Geometry(real_geo, material));
            if let Some(trans) = remaining_transformation {
                elem = wb.alloc(SceneElement::Transformation(elem, trans))
            }
            if self.0.moving_animation != Dir3::ZERO {
                elem = wb.alloc(SceneElement::Animation(elem, self.0.moving_animation))
            }
            elements.push(elem);
            if *is_poi && remaining_transformation.is_none() && wsd.is_none() {
                wsd = real_geo.get_world_scattering_provider()
            }
        }
        for child in &self.0.children {
            let (e, child_wsd) = child.finish_internal(wb, time_range, &full_trans);
            elements.push(e);
            if wsd.is_none() && child_wsd.is_some() {
                wsd = child_wsd;
            }
        }
        let root = if elements.len() != 1 {
            wb.alloc(SceneElement::Group(elements))
        } else {
            elements[0]
        };
        (root, wsd)
    }
}
