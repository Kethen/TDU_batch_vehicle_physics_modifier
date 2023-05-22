use xml_dom::level2::*;
use xml_dom::level2::convert::*;
use xml_dom::parser::read_xml;

mod tdudec;

fn main(){
	simple();
	//test_tdudec(true);
	//test_tdudec(false);
}

fn modify_value(value:f32, modifier:f32) -> f32{
	if value > 0f32{
		return value * (1f32 + modifier);
	}else{
		return value * (1f32 - modifier);
	}
}

fn simple(){
	let encrypted_content = match std::fs::read("AIConfig.xml"){
		Ok(f) => f,
		Err(e) => panic!("cannot read file, {}", e),
	};

	let mut decrypted_content = tdudec::decrypt_others(&encrypted_content);
	while decrypted_content[decrypted_content.len()-1] == 0x0{
		decrypted_content.pop();
	}

	let decrypted_string = match std::str::from_utf8(&decrypted_content[..]){
		Ok(d) => d,
		Err(e) => panic!("cannot convert decrypted content into string, {}", e),
	};

	let mut document_node = match read_xml(decrypted_string){
		Ok(d) => d,
		Err(e) => panic!("cannot parse decrypted file as xml, {}\n{}",e, decrypted_string),
	};

	let document = match as_document_mut(&mut document_node){
		Ok(d) => d,
		Err(e) => panic!("cannot cast RefNode to MutRefDocument"),
	};

	for child in document.child_nodes(){
		if child.node_name().local_name().eq("AICONFIG"){
			for child in child.child_nodes(){
				if child.node_name().local_name().eq("VEHICLE_PHYSICS"){
					for mut child in child.child_nodes(){
						let database_id = child.get_attribute("database_id").unwrap();
						let log_max_speed = child.get_attribute("log_max_speed");
						let braking_dist_c1 = child.get_attribute("braking_dist_c1").unwrap().parse::<f32>().unwrap();
						let braking_dist_c2 = child.get_attribute("braking_dist_c2").unwrap().parse::<f32>().unwrap();
						let turning_speed_c1 = child.get_attribute("turning_speed_c1").unwrap().parse::<f32>().unwrap();
						let turning_speed_c2 = child.get_attribute("turning_speed_c2").unwrap().parse::<f32>().unwrap();
						let handling_mark = child.get_attribute("handling_mark").unwrap().parse::<f32>().unwrap();

						let modifier = 3;
						child.set_attribute("braking_dist_c1", &modify_value(braking_dist_c1, 0.05).to_string());
						child.set_attribute("braking_dist_c2", &modify_value(braking_dist_c2, 0.05).to_string());
						child.set_attribute("turning_speed_c1", &modify_value(turning_speed_c1, -0.25).to_string());
						child.set_attribute("turning_speed_c2", &modify_value(turning_speed_c2, -0.25).to_string());
						match log_max_speed{
							Some(_) => {child.remove_attribute("log_max_speed").unwrap()},
							None => {},
						};
					}
					let new_xml = document_node.to_string().as_bytes().to_vec();
					let new_xml_encrypted = tdudec::encrypt_others(&new_xml);
					std::fs::write("AIConfig.xml.modified", &new_xml_encrypted);
					let new_xml = tdudec::decrypt_others(&new_xml_encrypted);
					std::fs::write("AIConfig.xml.modified.plain", &new_xml);
					return;
				}
			}
			panic!("VEHICLE_PHYSICS node not found");
		}
	}
	panic!("AICONFIG node not found");
}

fn test_tdudec(save:bool) {
	let path = if save{
		"playersave"
	}else{
		"AIConfig.xml"
	};

	let encrypted_content = match std::fs::read(path){
		Ok(f) => f,
		Err(e) => panic!("cannot read file, {}", e),
	};

	let d = if save{
		let decrypted_content = tdudec::decrypt_save(&encrypted_content);
		let encrypted_content = tdudec::encrypt_save(&decrypted_content);
		tdudec::decrypt_save(&encrypted_content)
	}else{
		let decrypted_content = tdudec::decrypt_others(&encrypted_content);
		let encrypted_content = tdudec::encrypt_others(&decrypted_content);
		tdudec::decrypt_others(&encrypted_content)
	};

	match std::fs::write(format!("{}.plain", path), d){
		Ok(_) => (),
		Err(e) => panic!("cannot write plain text, {}", e),
	};
}
