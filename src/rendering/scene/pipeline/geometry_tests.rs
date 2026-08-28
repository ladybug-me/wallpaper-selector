use super::place_instances;
use crate::contracts::rendering::InstanceRaw;

#[test]
fn embedded_scene_origin() {
    let instance = InstanceRaw { rect: [100.0, 80.0, 30.0, 20.0], ..InstanceRaw::default() };
    let instances = [instance];
    let placed = place_instances(&instances, 2.0, [40.0, 25.0]);
    assert_eq!(placed[0].rect, [280.0, 210.0, 60.0, 40.0]);
}
