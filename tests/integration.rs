#[cfg(test)]
mod integration {
    mod support;
    mod create;
    mod start;
    mod state;
    mod kill;
    mod delete;

    #[test]
    fn valid_life_cycle() {
        let project_path = support::create_project_path();
        let uuid = support::generate_uuid();

        support::initialize_test(&project_path);

        // if !create::exec(&project_path, &uuid.to_string()) {
        //     support::clean_test(&project_path);
        //     panic!("This operation must create a new container.");
        // }
        // if !state::exec(&project_path, &uuid.to_string()) {
        //     support::clean_test(&project_path);
        //     todo!();
        // }
        // if !start::exec(&project_path, &uuid.to_string()) {
        //     support::clean_test(&project_path);
        //     todo!();
        // }
        // if !state::exec(&project_path, &uuid.to_string()) {
        //     support::clean_test(&project_path);
        //     todo!();
        // }
        // if !state::exec(&project_path, &uuid.to_string()) {
        //     support::clean_test(&project_path);
        //     todo!();
        // }
        // if !kill::exec(&project_path, &uuid.to_string()) {
        //     support::clean_test(&project_path);
        //     todo!();
        // }
        // if !delete::exec(&project_path, &uuid.to_string()) {
        //     support::clean_test(&project_path);
        //     todo!();
        // }

        // support::clean_test(&project_path);
    }

    #[test]
    fn create() {
        let project_path = support::create_project_path();
        let uuid = support::generate_uuid();

        if create::exec(&project_path, "") {
            support::clean_test(&project_path);
            panic!("This operation MUST generate an error if it is not provided a path to the bundle and the container ID to associate with the container.");
        }
        if !create::exec(&project_path, &uuid.to_string()) {
            support::clean_test(&project_path);
            panic!("This operation MUST create a new container.");
        }
        if create::exec(&project_path, &uuid.to_string()) {
            support::clean_test(&project_path);
            panic!("If the ID provided is not unique across all containers within the scope of the runtime, or is not valid in any other way, the implementation MUST generate an error and a new container MUST NOT be created.");
        }

        support::clean_test(&project_path);
    }
}
