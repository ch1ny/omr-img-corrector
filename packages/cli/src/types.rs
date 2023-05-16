#[derive(Debug)]
pub struct ProjectionParams {
    pub projection_max_angle: u16,
    pub projection_angle_step: f64,
    pub projection_max_width: i32,
    pub projection_max_height: i32,
}

#[derive(Debug)]
pub struct EdgesDetectionParams {
    pub min_line_length: f64,
    pub max_line_gap: f64,
}

#[derive(Debug)]
pub struct FourierTransformParams {
    pub min_line_length: f64,
    pub max_line_gap: f64,
}

#[derive(Debug)]
pub struct Params {
    projection: Option<ProjectionParams>,
    edges: Option<EdgesDetectionParams>,
    fourier: Option<FourierTransformParams>,
}
impl Params {
    pub fn new() -> Self {
        Self {
            projection: None,
            edges: None,
            fourier: None,
        }
    }

    pub fn clone(&self) -> Self {
        let new_projection: Option<ProjectionParams> = if self.get_projection_params().is_none() {
            None
        } else {
            let p = self.get_projection_params().as_ref().unwrap();
            Some(ProjectionParams {
                projection_max_angle: p.projection_max_angle,
                projection_angle_step: p.projection_angle_step,
                projection_max_width: p.projection_max_width,
                projection_max_height: p.projection_max_height,
            })
        };

        let new_edges: Option<EdgesDetectionParams> = if self.get_edges_params().is_none() {
            None
        } else {
            let p = self.get_edges_params().as_ref().unwrap();
            Some(EdgesDetectionParams {
                min_line_length: p.min_line_length,
                max_line_gap: p.max_line_gap,
            })
        };

        let new_fourier: Option<FourierTransformParams> = if self.get_fourier_params().is_none() {
            None
        } else {
            let p = self.get_fourier_params().as_ref().unwrap();
            Some(FourierTransformParams {
                min_line_length: p.min_line_length,
                max_line_gap: p.max_line_gap,
            })
        };

        Self {
            projection: new_projection,
            edges: new_edges,
            fourier: new_fourier,
        }
    }

    pub fn set_projection_params(&mut self, params: ProjectionParams) {
        self.projection = Some(params);
    }

    pub fn get_projection_params(&self) -> &Option<ProjectionParams> {
        &self.projection
    }

    pub fn set_edges_params(&mut self, params: EdgesDetectionParams) {
        self.edges = Some(params);
    }

    pub fn get_edges_params(&self) -> &Option<EdgesDetectionParams> {
        &self.edges
    }

    pub fn set_fourier_params(&mut self, params: FourierTransformParams) {
        self.fourier = Some(params);
    }

    pub fn get_fourier_params(&self) -> &Option<FourierTransformParams> {
        &self.fourier
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Method {
    Default,
    ProjectionOnly,
    EdgesDetectionOnly,
    FourierTransformOnly,
}
