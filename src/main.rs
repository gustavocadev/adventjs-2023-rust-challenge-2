fn manufacture(gifs: Vec<&str>, materials: String) -> Vec<&str> {
  let mut possible_gifs: Vec<&str> = vec![];

  for gif in gifs {
    let can_create_gif = gif.chars().all(|gif_char| {
      let exists_material = materials.chars().any(|material| gif_char == material);
      if exists_material {
        return exists_material;
      }
      exists_material
    });

    if can_create_gif {
      possible_gifs.push(gif);
    }
  }

  return possible_gifs;
}

fn main() {
  let gifs: Vec<&str> = vec!["tren", "oso", "pelota"];
  let materials = String::from("tronesa");

  println!("{:?}", manufacture(gifs, materials));
}
