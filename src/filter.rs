use crate::WebcamFacialData;

pub struct WebcamFacialDataFiltered(Vec<WebcamFacialData>, u32, SmoothingFilterType);

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum SmoothingFilterType {
    MeanMedian,
    LowPass(f32), // LowPass filter with alpha parameter
    NoFilter,
}

impl WebcamFacialDataFiltered {
    pub fn new(length: u32, filter_type: SmoothingFilterType) -> Self {
        Self(Vec::new(), length, filter_type)
    }

    #[allow(unused)]
    pub fn push(&mut self, data: WebcamFacialData) {
        if self.0.len() >= self.1 as usize {
            self.0.remove(0); // Remove the oldest element
        }
        self.0.push(data);
    }

    #[allow(unused)]
    pub fn get(&mut self) -> WebcamFacialData {
        if self.0.len() == 0 {
            self.push(WebcamFacialData::default());
        }
        match self.2 {
            SmoothingFilterType::MeanMedian => return self.mean_median_filter(),
            SmoothingFilterType::LowPass(alpha) => return self.low_pass_filter(alpha),
            SmoothingFilterType::NoFilter => {
                return WebcamFacialData {
                    center_x: self.0[self.0.len() - 1].center_x,
                    center_y: self.0[self.0.len() - 1].center_y,
                    x: self.0[self.0.len() - 1].x,
                    y: self.0[self.0.len() - 1].y,
                    width: self.0[self.0.len() - 1].width,
                    height: self.0[self.0.len() - 1].height,
                    score: self.0[self.0.len() - 1].score,
                }
            }
        }
    }

    #[allow(unused)]
    fn low_pass_filter(&self, alpha: f32) -> WebcamFacialData {
        let mut filtered_data = WebcamFacialData::default();
        for data in &self.0 {
            filtered_data.center_x += alpha * (data.center_x - filtered_data.center_x);
            filtered_data.center_y += alpha * (data.center_y - filtered_data.center_y);
            filtered_data.x += alpha * (data.x - filtered_data.x);
            filtered_data.y += alpha * (data.y - filtered_data.y);
            filtered_data.width += alpha * (data.width - filtered_data.width);
            filtered_data.height += alpha * (data.height - filtered_data.height);
            filtered_data.score += alpha * (data.score - filtered_data.score);
        }

        filtered_data
    }

    #[allow(unused)]
    fn mean_median_filter(&self) -> WebcamFacialData {
        let num_elements = self.0.len();

        let mut center_x_sum = 0.0;
        let mut center_y_sum = 0.0;
        let mut x_sum = 0.0;
        let mut y_sum = 0.0;
        let mut width_sum = 0.0;
        let mut height_sum = 0.0;
        let mut score_sum = 0.0;

        for data in &self.0 {
            center_x_sum += data.center_x;
            center_y_sum += data.center_y;
            x_sum += data.x;
            y_sum += data.y;
            width_sum += data.width;
            height_sum += data.height;
            score_sum += data.score;
        }

        WebcamFacialData {
            center_x: center_x_sum / num_elements as f32,
            center_y: center_y_sum / num_elements as f32,
            x: x_sum / num_elements as f32,
            y: y_sum / num_elements as f32,
            width: width_sum / num_elements as f32,
            height: height_sum / num_elements as f32,
            score: score_sum / num_elements as f32,
        }
    }
}
