```rust
Some(
    [
        "major_connected_component",
        "major_connected_component.max_hole_ilen",
        "0.",
        "major_connected_component.max_hole_ilen == 0.",
        "major_connected_component.max_hole_ilen == 0.",
        "require major_connected_component.max_hole_ilen == 0.",
        "simple_seven_match",
        "simple_seven_match.norm",
        "let simple_match_norm = simple_seven_match.norm",
        "simple_match_norm",
        "1.0",
        "simple_match_norm < 1.0",
        "simple_match_norm < 1.0",
        "simple_seven_match",
        "simple_seven_match.matches",
        "0",
        "simple_seven_match.matches[0]",
        "simple_seven_match.matches[0] be Some(_)",
        "require simple_seven_match.matches[0] be Some(_)",
        "major_connected_component",
        "major_connected_component.upper_mass",
        "major_connected_component",
        "major_connected_component.lower_mass",
        "major_connected_component.upper_mass - major_connected_component.lower_mass",
        "let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass",
        "upper_excess",
        "10.",
        "upper_excess < 10.",
        "upper_excess < 10.",
        "simple_seven_match",
        "simple_seven_match.matches",
        "0",
        "simple_seven_match.matches[0]",
        "simple_seven_match.matches[0]!",
        "simple_seven_match.matches[0]!.end_tangent()",
        "let end_tangent = simple_seven_match.matches[0]!.end_tangent()",
        "end_tangent",
        "end_tangent.y",
        "let a = end_tangent.y",
        "a",
        "7.0",
        "-7.0",
        "a < -7.0",
        "a < -7.0",
        "require a < -7.0",
        "if upper_excess < 10.:\n            let end_tangent = simple_seven_match.matches[0]!.end_tangent()\n            let a = end_tangent.y\n            require a < -7.0",
        "OneVsAll::Yes",
        "return OneVsAll::Yes",
        "if simple_match_norm < 1.0:\n        require simple_seven_match.matches[0] be Some(_)\n        let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n        if upper_excess < 10.:\n            let end_tangent = simple_seven_match.matches[0]!.end_tangent()\n            let a = end_tangent.y\n            require a < -7.0\n        return OneVsAll::Yes",
        "simple_match_norm",
        "4.0",
        "simple_match_norm < 4.0",
        "simple_match_norm < 4.0",
        "major_connected_component",
        "major_connected_component.upper_mass",
        "major_connected_component",
        "major_connected_component.lower_mass",
        "major_connected_component.upper_mass - major_connected_component.lower_mass",
        "let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass",
        "upper_excess",
        "10.",
        "upper_excess > 10.",
        "upper_excess > 10.",
        "require upper_excess > 10.",
        "OneVsAll::Yes",
        "return OneVsAll::Yes",
        "if simple_match_norm < 4.0:\n        let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n        require upper_excess > 10.\n        return OneVsAll::Yes",
        "special_seven_match",
        "special_seven_match.matches",
        "0",
        "special_seven_match.matches[0]",
        "special_seven_match.matches[0] be Some(_)",
        "require special_seven_match.matches[0] be Some(_)",
        "special_seven_match",
        "special_seven_match.others",
        "let others = special_seven_match.others",
        "false",
        "false",
        "require false",
        "OneVsAll::Yes",
        "OneVsAll::Yes",
        "require major_connected_component.max_hole_ilen == 0.\n    let simple_match_norm = simple_seven_match.norm\n    if simple_match_norm < 1.0:\n        require simple_seven_match.matches[0] be Some(_)\n        let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n        if upper_excess < 10.:\n            let end_tangent = simple_seven_match.matches[0]!.end_tangent()\n            let a = end_tangent.y\n            require a < -7.0\n        return OneVsAll::Yes\n    if simple_match_norm < 4.0:\n        let upper_excess = major_connected_component.upper_mass - major_connected_component.lower_mass\n        require upper_excess > 10.\n        return OneVsAll::Yes\n    require special_seven_match.matches[0] be Some(_)\n    let others = special_seven_match.others\n    require false\n    OneVsAll::Yes",
    ],
)
```