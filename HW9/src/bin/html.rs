extern crate rand;
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct Circle {
    x: i32,
    y: i32,
    r: i32,
}

#[derive(Debug)]
struct Layer {
    name: String,
    color: String,
    objects: Vec<Circle>,
}


fn gen_obj_layer_list<R: Rng>(rng: &mut R, n: usize) -> Vec<Layer> {
    let mut layers = Vec::with_capacity(n);

    for i in 0..n {
        let name = format!("Layer {}", i + 1);
        let color = format!("#{:02X}{:02X}{:02X}{:02X}", rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>());

        let mut objects = Vec::new();
        let num_circles = rng.gen_range(20..=50);

        for _ in 0..num_circles {
            let x = rng.gen_range(-100..=100);
            let y = rng.gen_range(-100..=100);
            let r = rng.gen_range(-10..=20);

            let circle = Circle { x, y, r };
            objects.push(circle);
        }

        let layer = Layer { name, color, objects };
        layers.push(layer);
    }

    layers
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_gen_obj_layer_list() {
        let mut rng = thread_rng();
        let n = 5; 

        let layers = gen_obj_layer_list(&mut rng, n);

        assert_eq!(layers.len(), n);

        for layer in &layers {
            assert!(layer.name.starts_with("Layer "));
            assert!(layer.name.len() > "Layer ".len());

            assert_eq!(layer.color.len(), 9);
            assert!(layer.color.starts_with("#"));

            for circle in &layer.objects {
                assert!((-100..=100).contains(&circle.x));
                assert!((-100..=100).contains(&circle.y));
                assert!((-10..=20).contains(&circle.r));
            }
        }
    }
}




//question 1.2




fn cal_average_area(layers: &[Layer]) -> Vec<(String, f64)> {
    let mut result = Vec::new();

    for layer in layers {
        let mut total_area = 0.0;

        for circle in &layer.objects {
            let circle_area = std::f64::consts::PI * (circle.r.pow(2) as f64);
            total_area += circle_area;
        }

        let average_area = if layer.objects.is_empty() {
            0.0 
        } else {
            total_area / layer.objects.len() as f64
        };

        result.push((layer.name.clone(), average_area));
    }

    result
}


#[test]
fn test_cal_average_area() {
    let layer1 = Layer {
        name: String::from("Layer 1"),
        color: String::from("#RRGGBBAA"),
        objects: vec![
            Circle { x: 0, y: 0, r: 10 },
            Circle { x: 0, y: 0, r: 5 },
        ],
    };

    let layer2 = Layer {
        name: String::from("Layer 2"),
        color: String::from("#RRGGBBAA"),
        objects: vec![Circle { x: 0, y: 0, r: 15 }],
    };

    let layers = vec![layer1, layer2];

    let result = cal_average_area(&layers);

    assert_eq!(result.len(), 2);

    assert_eq!(result[0], (String::from("Layer 1"), 196.34954084936209)); // Corrected expected value
    assert_eq!(result[1], (String::from("Layer 2"), 706.8583470577034));
}



fn main() -> Result<(), Box<dyn Error>> {
    let n = 10;
    let mut rng = rand::thread_rng();
    let layers = gen_obj_layer_list(&mut rng, n);

    let html_file_path = "layers.html";

    let mut file = File::create(html_file_path)?;

    write!(file, "<html><head><title>Layers Data</title></head><body>\n")?;
    write!(file, "<table border=\"1\">\n")?;

    write!(
        file,
        "<tr><th>Layer Name</th><th>Color</th><th>Circle Count</th><th>Average Area</th><th>Min Area</th><th>Max Area</th></tr>\n"
    )?;

    for layer in &layers {
        let average_area = layer
            .objects
            .iter()
            .map(|circle| std::f64::consts::PI * (circle.r.pow(2) as f64))
            .sum::<f64>()
            / layer.objects.len() as f64;

        let min_area = layer
            .objects
            .iter()
            .map(|circle| std::f64::consts::PI * (circle.r.pow(2) as f64))
            .fold(f64::INFINITY, f64::min);

        let max_area = layer
            .objects
            .iter()
            .map(|circle| std::f64::consts::PI * (circle.r.pow(2) as f64))
            .fold(f64::NEG_INFINITY, f64::max);

        write!(
            file,
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{:.2}</td><td>{:.2}</td><td>{:.2}</td></tr>\n",
            &layer.name, &layer.color, layer.objects.len(), average_area, min_area, max_area
        )?;
    }

    write!(file, "</table>\n")?;
    write!(file, "</body></html>\n")?;

    println!("Layers data saved to {}", html_file_path);

    Ok(())
}