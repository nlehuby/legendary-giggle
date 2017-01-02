extern crate osmpbfreader;
extern crate csv;

fn wanted(obj: &osmpbfreader::OsmObj) -> bool{
    match *obj {
        //osmpbfreader::OsmObj::Relation(ref rel) => rel.id == 7444,//id of relation for Paris
        //osmpbfreader::OsmObj::Relation(ref rel) => rel.id == 1066570,//id of relation for bus 67
        osmpbfreader::OsmObj::Relation(ref rel) => rel.id == 1257187,//id of relation for bus 57
        _ => false,
    }
}

fn is_route_master(obj: &osmpbfreader::OsmObj) -> bool {
    obj.relation()
        .and_then(|r| r.tags.get("type"))
        .map_or(false, |v| ["route_master"].contains(&v.as_str()))
}

fn main() {
    let filename = std::env::args_os().nth(1).unwrap();
    let path = std::path::Path::new(&filename);
    let r = std::fs::File::open(&path).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(r);
    let objects = osmpbfreader::get_objs_and_deps(&mut pbf, wanted).unwrap();

    let mut wtr = csv::Writer::from_file("relations_members.csv").unwrap();
        wtr.encode(("member_id", "member_role", "parent_relation_id"))
            .unwrap();

    println!("The relation is composed of {:?} items", objects.len());
    for (_, elem) in objects{
        if elem.relation().is_some() && is_route_master(&elem) {
            println!("{:?}", elem.relation().unwrap().tags.get("name").unwrap());
            match elem {
                osmpbfreader::OsmObj::Relation(relation) => {

                    for member in &relation.refs {
                        match member.member{
                            osmpbfreader::OsmId::Relation(rel_id) => {
                                //println!("{:?} , {:?} , {:?}", &relation.id, rel_id, member.role);
                                wtr.encode((&relation.id, &member.role,rel_id)).unwrap();
                            },
                            _ => {;}
                        }
                    }
                }
                _ => {;}
            }
        }
    }
}
