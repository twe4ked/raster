// https://en.wikipedia.org/wiki/Truevision_TGA

use std::collections::HashMap;
use std::convert::TryInto;

use crate::vec3::Vec3;

pub struct TgaImage {
    pub pixels: HashMap<(usize, usize), Vec3>,
    pub width: usize,
    pub height: usize,
}

pub fn parse(input: &[u8]) -> TgaImage {
    // Length 	    Field name 	                Description
    // 1 byte 	    ID length 	                Length of the image ID field
    // 1 byte 	    Color map type 	            Whether a color map is included
    // 1 byte 	    Image type 	                Compression and color types
    // 5 bytes 	    Color map specification 	  Describes the color map
    // 10 bytes 	  Image specification 	      Image dimensions and format
    let (id_length, remaining) = input.split_at(1);
    assert_eq!(0, id_length[0]);
    let (color_map_type, remaining) = remaining.split_at(1);
    // Image file contains no color map
    assert_eq!(0, color_map_type[0]);
    let (image_type, remaining) = remaining.split_at(1);
    // run-length encoded true-color image
    assert_eq!(10, image_type[0]);

    // Color map specification:
    //      First entry index (2 bytes): index of first color map entry that is included in the file
    //      Color map length (2 bytes): number of entries of the color map that are included in the file
    //      Color map entry size (1 byte): number of bits per pixel
    let (color_map_specification, remaining) = remaining.split_at(5);
    let first_entry_index = u16::from_le_bytes(color_map_specification[0..2].try_into().unwrap());
    assert_eq!(0, first_entry_index);
    let color_map_length = u16::from_le_bytes(color_map_specification[2..4].try_into().unwrap());
    assert_eq!(0, color_map_length);
    let color_map_entry_size = color_map_specification[4];
    assert_eq!(0, color_map_entry_size);

    // Image specification:
    //      X-origin (2 bytes): absolute coordinate of lower-left corner for displays where origin is at the lower left
    //      Y-origin (2 bytes): as for X-origin
    //      Image width (2 bytes): width in pixels
    //      Image height (2 bytes): height in pixels
    //      Pixel depth (1 byte): bits per pixel
    //      Image descriptor (1 byte): bits 3-0 give the alpha channel depth, bits 5-4 give direction
    let (image_specification, remaining) = remaining.split_at(10);
    let x_origin = u16::from_le_bytes(image_specification[0..2].try_into().unwrap());
    assert_eq!(0, x_origin);
    let y_origin = u16::from_le_bytes(image_specification[2..4].try_into().unwrap());
    assert_eq!(0, y_origin);
    let image_width = u16::from_le_bytes(image_specification[4..6].try_into().unwrap()) as usize;
    let image_height = u16::from_le_bytes(image_specification[6..8].try_into().unwrap()) as usize;
    let pixel_depth = image_specification[8];
    assert_eq!(24, pixel_depth);
    let image_descriptor = image_specification[9];
    assert_eq!(0, image_descriptor);

    // Length 	                            Field 	        Description
    // From image ID length field 	        Image ID 	      Optional field containing identifying information
    // From color map specification field 	Color map data  Look-up table containing color map data
    // From image specification field 	    Image data 	    Stored according to the image descriptor

    // NOTE: Image ID and Color map data are both empty so the rest of the data is "image data"
    let image_data = remaining;

    let mut image = TgaImage {
        pixels: HashMap::new(),
        width: image_width,
        height: image_height,
    };

    // Origin is the bottom left
    let mut x = 0;
    let mut y = image_height;

    let mut i = 0;
    let total_pixel_count = image_width * image_height;

    while image.pixels.len() < total_pixel_count {
        let packet_header = image_data[i];
        i += 1;

        // The count of pixels is the lower 7 bits + 1
        let pixel_count = (packet_header & 0x7f) + 1;

        // The high-order bit of the header is 1 for the run length packets,
        // and 0 for the raw,packets
        if packet_header & 0x80 != 0 {
            // Run length packet
            let color = color(&image_data[i..i + 3]);
            i += 3;

            for _ in 0..pixel_count {
                image.pixels.insert((x, y), color);
                x += 1;
                if x == image_width {
                    y -= 1;
                    x = 0;
                }
            }
        } else {
            // Raw packet
            for _ in 0..pixel_count {
                let color = color(&image_data[i..i + 3]);
                i += 3;

                image.pixels.insert((x, y), color);
                x += 1;
                if x == image_width {
                    y -= 1;
                    x = 0;
                }
            }
        }
    }

    image
}

fn color(input: &[u8]) -> Vec3 {
    assert_eq!(3, input.len());
    Vec3::new(input[2] as f32, input[1] as f32, input[0] as f32)
}
