extern crate osmpbfreader;
extern crate csv;

#[derive(Debug, Clone)]
pub struct LineToRoute {
    pub parent_id: String,
    pub member_id: String,
    pub member_role: String,
}

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

fn extract_children_from_route_master(route_master: &osmpbfreader::Relation) -> Option<LineToRoute> {
    // retourner plutôt un vecteur de parcours pour chaque ligne ?
    for member in &route_master.refs{
        match member.member {
            osmpbfreader::OsmId::Relation(rel_id) => {
                println!("{:?}, {:?}, {:?}", route_master.id, rel_id, member.role);
                Some(LineToRoute { parent_id: route_master.id.to_string(), member_id: rel_id.to_string(), member_role: member.role.to_string() });
                }
            _ => {;}
        }
    }

    //TODO
    Some(LineToRoute { parent_id: route_master.id.to_string(), member_id: route_master.refs[0].role.to_string(), member_role: route_master.refs[0].role.to_string() })

}

fn main() {
    let filename = std::env::args_os().nth(1).unwrap();
    let path = std::path::Path::new(&filename);
    let r = std::fs::File::open(&path).unwrap();
    let mut pbf = osmpbfreader::OsmPbfReader::new(r);
    let objects = osmpbfreader::get_objs_and_deps(&mut pbf, wanted).unwrap();

    /*let mut wtr = csv::Writer::from_file("relations_members.csv").unwrap();
        wtr.encode(("member_id", "member_role", "parent_relation_id"))
            .unwrap(); */

    let result:Vec<LineToRoute> = objects.values()
        .filter(|x| is_route_master(*x))
        .filter_map(osmpbfreader::OsmObj::relation)
        .filter_map(|relation| extract_children_from_route_master(relation))
        .collect();


    println!("{:?}", result);
}
