use ygodestiny_macros::settings;

settings! {
    set_rotation {
        #root: expander "Set rotation"
            "Remove pulled cards from the card pool after some rounds";

        adj1 = adjustment 0.0..100.0 / 1.0;
        @root keep_sets: spin -> usize [@adj1] "Rotation rounds"
            "Number of rounds to keep sets for";

        adj2 = adjustment from adj1;
        @root exclude_first: spin -> usize [@adj2] "Rotation delay"
            "Number of rounds to exclude from rotation";

        @root full_rotation: switch "Full rotation"
            "Remove all sets from the pool at once instead of one round at a time";
    }

    multi_choice {
        #root: expander "Multi choice"
            "Select between multiple packs each draft round";

        adj1 = adjustment 2.0..100.0 / 1.0 = 2.0;
        @root choices: spin -> usize [@adj1] "Choices"
            "Number of packs to choose from";

        adj2 = adjustment 1.0..1.0 / 1.0 = 1.0;
        #link adjustment adj2 to adj1 (val - 1.0);
        @root selections: spin -> usize [@adj2] "Selections"
            "Number of selections that have to be made";

        @root unify_choices: switch "Unify choices"
            "Use the same card groups to generate choices (in sets with small groups this might make the same card appear in every choice)";
    }
}
