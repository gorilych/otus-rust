use crate::{Device, Home, Result, Room, SmartSocket, Thermometr};

fn connect_to_smartsocket_mock() -> Device {
    let smartsocket_mock_addr = std::env::var_os("SMARTSOCKET_ADDR")
        .and_then(|os_string| os_string.into_string().ok())
        .and_then(|string| string.parse().ok())
        .unwrap_or_else(|| "127.0.0.1:55331".to_string());

    SmartSocket::connect(smartsocket_mock_addr)
        .expect("run smartsocket mock server from eamples")
        .into()
}

#[test]
// test for Дом имеет название и содержит несколько помещений
// here we test that pub field `name` is correctly set with `Home::new()`
fn home_has_name() {
    let home_hame = String::from("my home");
    let home = Home::new(home_hame.clone());
    assert_eq!(home_hame, home.name);
}

#[test]
// test for Библитотека позволяет запросить список помещений, добавлять и удалять помещения в доме.
// here we test that home has as many rooms as we expect after adding and removing it
fn home_has_rooms() -> Result<()> {
    let mut home = Home::new("my home".into());

    // test we can add rooms
    home.add_room("room1".into(), Room::default())?;
    home.add_room("room2".into(), Room::default())?;
    home.add_room("room3".into(), Room::default())?;

    // test that we have rooms we added
    let mut room_names: Vec<_> = home.room_names().collect();
    room_names.sort_unstable(); // we don't have any order requirements so here we sort before checking result
    match room_names[..] {
        ["room1", "room2", "room3"] => Ok(()),
        ref x => Err(format!("expected [room1, room2, room3], got {:?}", x)),
    }?;

    // test we can get rooms we added
    home.room("room1").ok_or("no room1")?;
    home.room("room2").ok_or("no room2")?;
    home.room("room3").ok_or("no room3")?;

    // test we can delete room
    home.delete_room("room2")?;

    // test that room was deleted
    let mut room_names: Vec<_> = home.room_names().collect();
    room_names.sort_unstable();
    match room_names[..] {
        ["room1", "room3"] => Ok(()),
        ref x => Err(format!("expected [room1, room3], got {:?}", x)),
    }?;

    // test we can get rooms we didn't delete
    home.room("room1").ok_or("no room1")?;
    home.room("room3").ok_or("no room3")?;

    // test we can't get room we deleted
    if home.room("room2").is_some() {
        return Err("Expected home.room(\"room2\") to return Err".into());
    };

    Ok(())
}

#[test]
// test for Помещение имеет уникальное название и содержит несколько устройств
// here we test that home does not allow to add room with not unique name
fn home_room_non_unique_name_isnt_allowed() {
    let mut home = Home::new("my home".into());

    home.add_room("room1".into(), Room::default()).ok(); // ok() to ignore result

    // next add_room call with the same name should return err
    assert!(home.add_room("room1".into(), Room::default()).is_err());
}

#[test]
// test for Устройство имеет уникальное в рамках помещения название, тип и описание.
// here we test that room does not allow non-unique device names
fn room_device_non_unique_name_isnt_allowed() {
    let mut room = Room::default();

    room.add_device("device1".into(), "".into(), Thermometr::new().into())
        .ok(); // ok() to ignore result

    // next add_device call with the same name should return err
    assert!(room
        .add_device("device1".into(), "".into(), connect_to_smartsocket_mock())
        .is_err());
}

#[test]
// test for Устройство имеет уникальное в рамках помещения название, тип и описание.
// here we test that room does not allow non-unique device types
fn room_device_non_unique_type_isnt_allowed() {
    let mut room = Room::default();

    room.add_device("device1".into(), "".into(), Thermometr::new().into())
        .ok(); // ok() to ignore result

    // next add_device call with the same type should return err
    assert!(room
        .add_device("device2".into(), "".into(), Thermometr::new().into())
        .is_err());
}

#[test]
// test for Библтотека позволяет добавлять, получать и удалять любое устройство в доме. Получать список устройств в помещении.
// here we test we have as expected number of devices in a room
fn room_has_devices() -> Result<()> {
    let mut room = Room::default();

    // test we can add devices
    room.add_device("thermo".into(), "".into(), Thermometr::new().into())?;
    room.add_device("socket".into(), "".into(), connect_to_smartsocket_mock())?;

    // test that we have devices we added
    let mut device_names: Vec<_> = room.device_names().collect();
    device_names.sort_unstable(); // we don't have any order requirements so here we sort before checking result
    match device_names[..] {
        ["socket", "thermo"] => Ok(()),
        ref x => Err(format!("expected [socket, thermo], got {:?}", x)),
    }?;

    // test we can get devices we added
    room.device("thermo").ok_or("no thermo")?;
    room.device("socket").ok_or("no socket")?;

    // test we can delete device
    room.delete_device("thermo")?;

    // test that device was deleted
    let device_names: Vec<_> = room.device_names().collect();
    match device_names[..] {
        ["socket"] => Ok(()),
        ref x => Err(format!("expected [socket], got {:?}", x)),
    }?;

    // test we can get device we didn't delete
    room.device("socket").ok_or("no socket")?;

    // test we can't get device we deleted
    if room.device("thermo").is_some() {
        return Err("Expected room.device(\"thermo\") to return Err".into());
    };

    Ok(())
}

#[test]
//test for Типы устройств: термометр, умная розетка.
//here we test we can create device with type thermometr
fn create_thermometr() -> Result<()> {
    match Thermometr::new().into() {
        Device::Thermometr(_) => Ok(()),
        device => Err(format!("Expected thermometr, got {:?}", device).into()),
    }
}

#[test]
//test for Типы устройств: термометр, умная розетка.
//here we test we can create device with type smartsocket
fn create_smartsocket() -> Result<()> {
    match connect_to_smartsocket_mock() {
        Device::SmartSocket(_) => Ok(()),
        device => Err(format!("Expected smartsocket, got {:?}", device).into()),
    }
}

#[test]
// test for Термометр позволяет узнать температуру
// here we just test that functions do not panic, cause we don't have requriements
fn thermometr_functions() {
    let d: Device = Thermometr::new().into();

    d.state().ok(); // shouldn't panic

    if let Device::Thermometr(t) = d {
        t.temperature().ok(); // shouldn't panic
    } else {
        unreachable!("d must be Device::Thermometr")
    };
}

#[test]
// test for Умная розетка позволяет включать и выключать себя. Предоставляет информацию о текущем состоянии и потребляемой мощности.
// here we just test that functions do not panic, cause we don't have requriements
fn smartsocket_functions() {
    let d: Device = connect_to_smartsocket_mock();

    d.state().ok(); // shouldn't panic

    if let Device::SmartSocket(mut s) = d {
        s.switch(true).ok(); // shouldn't panic
        s.switch(false).ok(); // shouldn't panic
        s.state().ok(); // shouldn't panic
    } else {
        unreachable!("d must be Device::SmartSocket")
    };
}

#[test]
// test for Библиотека позволяет строить отчёт о состоянии всех устройств в доме.
// here we just test that functions do not panic, cause we don't have requriements
fn home_state_functions() {
    let mut home = Home::new("my home".into());
    let room_names = ["room1", "room2", "room3"];

    for rn in room_names {
        home.add_room(rn.into(), Room::default()).ok();

        home.room(rn)
            .unwrap()
            .add_device("thermo".into(), "".into(), Thermometr::new().into())
            .ok();

        home.room(rn)
            .unwrap()
            .add_device("socket".into(), "".into(), connect_to_smartsocket_mock())
            .ok();
    }

    home.state(); // shouldn't panic
}