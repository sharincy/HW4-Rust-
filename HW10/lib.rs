fn vflip(img: &[String]) -> Vec<String> {
    img.iter().rev().cloned().collect()
}

fn hflip(img: &[String]) -> Vec<String> {
    let max_len = img.iter().map(|s| s.len()).max().unwrap_or(0);
    img.iter()
        .map(|line| {
            let reversed_line: String = line.chars().rev().collect();
            format!("{:>width$}", reversed_line, width = max_len)
        })
        .collect()
}

#[test] 
fn test_img_flip() { 
    let emp = ["".to_string(); 0]; 
    assert_eq!(vflip(&emp), [""; 0]); 
    assert_eq!(hflip(&emp), [""; 0]); 
 
    let data = [ 
        "<--", 
        "#####", 
        "<==" 
    ].map(|v| v.to_string()); 
 
    assert_eq!( 
        vflip(&data), [ 
            "<==", 
            "#####", 
            "<--" 
        ]); 
    assert_eq!( 
        hflip(&data), [ 
            "  --<", 
            "#####", 
            "  ==<" 
        ]); 
}

//------------question 1^



fn vcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for line in img1.iter() {
        result.push(line.clone());
    }
    for line in img2.iter() {
        result.push(line.clone());
    }
    result
}

fn hcat(img1: &[String], img2: &[String]) -> Vec<String> {
    if img1.is_empty() && img2.is_empty() {
        return Vec::new();
    }

    let max_len = img1.iter().map(|s| s.len()).max().unwrap_or(0);
    let mut result = Vec::new();

    for (line1, line2) in img1.iter().zip(img2.iter()) {
        let combined_line = format!("{:<width$}{}", line1, line2, width = max_len);
        result.push(combined_line);
    }

    for line in img1.iter().skip(img2.len()) {
        result.push(line.clone());
    }

    for line in img2.iter().skip(img1.len()) {
        let combined_line = format!("{:<width$}{}", "", line, width = max_len);
        result.push(combined_line);
    }

    result
}




#[test] 
fn test_img_cat() { 
    let emp = ["".to_string(); 0]; 
    assert_eq!(vcat(&emp, &emp), [""; 0]); 
    assert_eq!(hcat(&emp, &emp), [""; 0]); 
 
    let data = [ 
        "<--", 
        "#####", 
        "<==" 
    ].map(|v| v.to_string()); 
    assert_eq!(vcat(&emp, &data), data); 
    assert_eq!(vcat(&data, &emp), data); 
 
    assert_eq!( 
        vcat(&data, &data), [ 
            "<--", 
            "#####", 
            "<==", 
            "<--", 
            "#####", 
            "<==" 
        ]); 
 
    assert_eq!( 
        hcat(&data, &data[..2]), [ 
            "<--  <--", 
            "##########", 
            "<==" 
        ]); 
    assert_eq!( 
        hcat(&data[..2], &data), [ 
            "<--  <--", 
            "##########", 
            "     <==" 
        ]); 
}