use std::u8;

use image::Rgb;

pub struct Bucket {
    colors: Vec<Rgb<u8>>
}

#[derive(PartialEq)]
enum Channel {
    Red,
    Green,
    Blue,
}

impl Bucket {
    pub fn new(colors: Vec<Rgb<u8>>) -> Self {
        Bucket { colors }
    }

    // function to find the greatest range of any channel
    fn find_greatest_range (&self) -> u8 {
        let mut r_min = u8::MAX;
        let mut r_max = 0;
        let mut g_min = u8::MAX;
        let mut g_max = 0;
        let mut b_min = u8::MAX;
        let mut b_max = 0;

        for color in &self.colors {
            // red
            if color[0] < r_min {
                r_min = color[0];
            } else if color[0] > r_max {
                r_max = color[0];
            }

            // green
            if color[1] < g_min {
                g_min = color[1];
            } else if color[1] > g_max {
                g_max = color[1];
            }

            // blue
            if color[2] < b_min {
                b_min = color[2];
            } else if color[2] > b_max {
                b_max = color[2];
            }
        }

        let r_dif = r_max - r_min;
        let g_dif = g_max - g_min;
        let b_dif = b_max - b_min;
        
        let max_dif = r_dif.max(g_dif).max(b_dif);
        
        return max_dif;

    }

    // function to find which channel (r, g, b) has the greatest range
    fn find_greatest_channel_range(&self) -> Channel {
        let mut r_min = u8::MAX;
        let mut r_max = 0;
        let mut g_min = u8::MAX;
        let mut g_max = 0;
        let mut b_min = u8::MAX;
        let mut b_max = 0;

        for color in &self.colors {
            // red
            if color[0] < r_min {
                r_min = color[0];
            } else if color[0] > r_max {
                r_max = color[0];
            }

            // green
            if color[1] < g_min {
                g_min = color[1];
            } else if color[1] > g_max {
                g_max = color[1];
            }

            // blue
            if color[2] < b_min {
                b_min = color[2];
            } else if color[2] > b_max {
                b_max = color[2];
            }
        }

        let r_dif = r_max - r_min;
        let g_dif = g_max - g_min;
        let b_dif = b_max - b_min;
        let largest_channel = 
            if r_dif >= g_dif && r_dif >= b_dif {
                Channel::Red
            } else if g_dif >= b_dif {
                Channel::Green
            } else {
                Channel::Blue
            };
        
        return largest_channel;
        
    }

    // function to sort the colors based off of a channel
    fn sort_by_channel(&mut self, channel: Channel) {
        match channel {
            Channel::Red => self.colors.sort_by_key(|c| c[0]),
            Channel::Green => self.colors.sort_by_key(|c| c[1]),
            Channel::Blue => self.colors.sort_by_key(|c| c[2]),
        }
        
    }

    fn split_half (&mut self) -> Bucket {
        let top_half = self.colors.split_off(self.colors.len()/2);
        return Bucket { colors: top_half };
    }

    fn mean_color (&self) -> Rgb<u8> {
        let mut r_sum = 0;
        let mut g_sum = 0;
        let mut b_sum = 0;
        
        for color in &self.colors {
            r_sum += color[0] as usize;
            g_sum += color[1] as usize;
            b_sum += color[2] as usize;
        }

        r_sum /= self.colors.len();
        g_sum /= self.colors.len();
        b_sum /= self.colors.len();

        let mean = Rgb([r_sum as u8, g_sum as u8, b_sum as u8]);

        return mean;
    }
}

pub fn generate_palette(colors: Vec<Rgb<u8>>) -> Vec<Rgb<u8>> {

    // create a list of buckets
    let mut buckets = Vec::new();

    // create the initial bucket
    let bucket = Bucket::new(colors);
    buckets.push(bucket);

    while buckets.len() < 15 {
        // find the bucket with the largest range in any channel
        let mut max_range_bucket_idx = 0;
        let mut max_range = 0;
        for (i, bucket) in buckets.iter().enumerate() {
            let bucket_range = bucket.find_greatest_range();
            if bucket_range > max_range {
                max_range_bucket_idx = i;
                max_range = bucket_range;
            }
        }

        // find the channel with the most range
        let max_range_channel = buckets[max_range_bucket_idx].find_greatest_channel_range();
        // temporarily remove the bucket to avoid double mutable borrow
        let mut bucket = buckets.remove(max_range_bucket_idx);
        // sort the bucket colors by the channel
        bucket.sort_by_channel(max_range_channel);
        // split the upper half into its own bucket
        let new_bucket = bucket.split_half();
        // put the modified bucket back
        buckets.insert(max_range_bucket_idx, bucket);
        // push the new bucket
        buckets.push(new_bucket);
    }

    // get the average color from each bucket
    let mut avg_colors = Vec::new();

    for bucket in buckets {
        avg_colors.push(bucket.mean_color());
    }

    return avg_colors;


}