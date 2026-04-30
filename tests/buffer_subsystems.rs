use fenirer::buffer::{BufferArena, BufferRegion, RegionRegistry, SharedBuffer, Slab};

#[test]
fn arena_write_read() {
    let mut a = BufferArena::with_capacity(128);
    let off = a.write(b"fenirer").unwrap();
    assert_eq!(a.slice(off, 7), b"fenirer");
}

#[test]
fn slab_acquire_release() {
    let mut s = Slab::<16>::with_slots(8);
    let i = s.acquire().unwrap();
    s.release(i);
}

#[test]
fn region_overlap() {
    let buf = SharedBuffer::with_capacity(64, 1);
    let r1 = BufferRegion::new("a", &buf, 0, 16);
    let r2 = BufferRegion::new("b", &buf, 8, 16);
    assert!(r1.overlaps(&r2));
}

#[test]
fn region_registry() {
    let mut reg = RegionRegistry::default();
    let buf = SharedBuffer::with_capacity(32, 1);
    reg.register(BufferRegion::new("hdr", &buf, 0, 8));
    assert_eq!(reg.len(), 1);
    reg.clear();
    assert_eq!(reg.len(), 0);
}
