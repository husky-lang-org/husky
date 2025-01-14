## `two_match` decl

```rust
Some(
    [
        "FermiMatchResult",
    ],
)
```

## `two_match` defn

```rust
Some(
    [
        "fermi_match",
        "major_concave_components",
        "left_cc_pattern",
        "right_cc_pattern",
        "down_cc_pattern",
        "[left_cc_pattern, right_cc_pattern,down_cc_pattern]",
        "fermi_match(major_concave_components,[left_cc_pattern, right_cc_pattern,down_cc_pattern])",
        "fermi_match(major_concave_components,[left_cc_pattern, right_cc_pattern,down_cc_pattern])",
        "fermi_match(major_concave_components,[left_cc_pattern, right_cc_pattern,down_cc_pattern])",
    ],
)
```

## `left_cc_pattern` decl

```rust
Some(
    [
        "ConcaveComponent",
        "~ConcaveComponent",
        "f32",
        "?f32",
    ],
)
```

## `left_cc_pattern` defn

```rust
Some(
    [
        "cc",
        "cc.displacement()",
        "let dp = cc.displacement()",
        "dp",
        "dp.y",
        "0.0",
        "dp.y < 0.0",
        "dp.y < 0.0",
        "require dp.y < 0.0",
        "dp",
        "dp.y",
        "dp.y",
        "let dp = cc.displacement()\n    require dp.y < 0.0\n    dp.y",
    ],
)
```

## `right_cc_pattern` decl

```rust
Some(
    [
        "ConcaveComponent",
        "~ConcaveComponent",
        "f32",
        "?f32",
    ],
)
```

## `right_cc_pattern` defn

```rust
Some(
    [
        "cc",
        "cc.displacement()",
        "let dp = cc.displacement()",
        "dp",
        "dp.y",
        "0.0",
        "dp.y > 0.0",
        "dp.y > 0.0",
        "require dp.y > 0.0",
        "dp",
        "dp.y",
        "dp.y",
        "let dp = cc.displacement()\n    require dp.y > 0.0\n    dp.y",
    ],
)
```

## `down_cc_pattern` decl

```rust
Some(
    [
        "ConcaveComponent",
        "~ConcaveComponent",
        "f32",
        "?f32",
    ],
)
```

## `down_cc_pattern` defn

```rust
Some(
    [
        "cc",
        "cc.displacement()",
        "let dp = cc.displacement()",
        "dp",
        "dp.x",
        "0.0",
        "dp.x > 0.0",
        "dp.x > 0.0",
        "require dp.x > 0.0",
        "dp",
        "dp.x",
        "dp.x",
        "let dp = cc.displacement()\n    require dp.x > 0.0\n    // require cc.relative_bounding_box.ymin()<0.2\n    dp.x",
    ],
)
```

## `is_two` decl

```rust
Some(
    [
        "OneVsAll",
        "MnistLabel",
        "OneVsAll MnistLabel",
        "MnistLabel::Two",
        "OneVsAll MnistLabel MnistLabel::Two",
    ],
)
```

## `is_two` defn

```rust
Some(
    [
        "major_concave_components",
        "major_concave_components.ilen()",
        "let cc_num = major_concave_components.ilen()",
        "major_connected_component",
        "major_connected_component.eff_holes",
        "let eff_holes = major_connected_component.eff_holes",
        "eff_holes",
        "eff_holes.matches",
        "1",
        "eff_holes.matches[1]",
        "eff_holes.matches[1] be None",
        "require eff_holes.matches[1] be None",
        "two_match",
        "two_match.matches",
        "0",
        "two_match.matches[0]",
        "let left_cc = two_match.matches[0]",
        "two_match",
        "two_match.matches",
        "1",
        "two_match.matches[1]",
        "let right_cc = two_match.matches[1]",
        "two_match",
        "two_match.matches",
        "2",
        "two_match.matches[2]",
        "let down_cc = two_match.matches[2]",
        "cc_num",
        "3",
        "cc_num<=3",
        "cc_num<=3",
        "require cc_num<=3",
        "major_connected_component",
        "major_connected_component.lower_mass",
        "major_connected_component",
        "major_connected_component.upper_mass",
        "major_connected_component.lower_mass - major_connected_component.upper_mass",
        "let lower_excess = major_connected_component.lower_mass - major_connected_component.upper_mass",
        "lower_excess",
        "10.0",
        "lower_excess > 10.0",
        "lower_excess > 10.0",
        "require lower_excess > 10.0",
        "cc_num",
        "2",
        "cc_num == 2",
        "cc_num == 2",
        "left_cc",
        "left_cc be Some(_)",
        "require left_cc be Some(_)",
        "right_cc",
        "right_cc be Some(_)",
        "require right_cc be Some(_)",
        "right_cc",
        "right_cc!",
        "right_cc!.angle_change",
        "let a = right_cc!.angle_change",
        "a",
        "180.0",
        "-180.0",
        "a>-180.0",
        "a>-180.0",
        "require a>-180.0",
        "left_cc",
        "left_cc!",
        "left_cc!.end_tangent()",
        "true",
        "left_cc!.end_tangent().angle(true)",
        "let end_tan = left_cc!.end_tangent().angle(true)",
        "left_cc",
        "left_cc!",
        "left_cc!.end_tangent()",
        "left_cc!.end_tangent().x",
        "let x = left_cc!.end_tangent().x",
        "left_cc",
        "left_cc!",
        "left_cc!.end_tangent()",
        "left_cc!.end_tangent().y",
        "let y = left_cc!.end_tangent().y",
        "left_cc",
        "left_cc!",
        "left_cc!.relative_bounding_box",
        "left_cc!.relative_bounding_box.ymax()",
        "let left_ymax = left_cc!.relative_bounding_box.ymax()",
        "left_cc",
        "left_cc!",
        "left_cc!.relative_bounding_box",
        "left_cc!.relative_bounding_box.ymin()",
        "let left_ymin = left_cc!.relative_bounding_box.ymin()",
        "left_ymax",
        "left_ymin",
        "left_ymax + left_ymin",
        "(left_ymax + left_ymin)",
        "2.0",
        "(left_ymax + left_ymin) / 2.0",
        "let left_mid_y = (left_ymax + left_ymin) / 2.0",
        "right_cc",
        "right_cc!",
        "right_cc!.relative_bounding_box",
        "right_cc!.relative_bounding_box.ymax()",
        "let right_ymax = right_cc!.relative_bounding_box.ymax()",
        "right_cc",
        "right_cc!",
        "right_cc!.relative_bounding_box",
        "right_cc!.relative_bounding_box.ymin()",
        "let right_ymin = right_cc!.relative_bounding_box.ymin()",
        "right_ymax",
        "right_ymin",
        "right_ymax + right_ymin",
        "(right_ymax + right_ymin)",
        "2.0",
        "(right_ymax + right_ymin) / 2.0",
        "let right_mid_y = (right_ymax + right_ymin) / 2.0",
        "left_mid_y",
        "right_mid_y",
        "left_mid_y >= right_mid_y",
        "left_mid_y >= right_mid_y",
        "require left_mid_y >= right_mid_y",
        "if cc_num == 2:\n        require left_cc be Some(_)\n        require right_cc be Some(_)\n        let a = right_cc!.angle_change\n        require a>-180.0\n        // get the end line of the cc\n        let end_tan = left_cc!.end_tangent().angle(true)\n        // require end_tan < -200.0\n        let x = left_cc!.end_tangent().x\n        let y = left_cc!.end_tangent().y\n\n        let left_ymax = left_cc!.relative_bounding_box.ymax()\n        let left_ymin = left_cc!.relative_bounding_box.ymin()\n        let left_mid_y = (left_ymax + left_ymin) / 2.0\n\n        let right_ymax = right_cc!.relative_bounding_box.ymax()\n        let right_ymin = right_cc!.relative_bounding_box.ymin()\n        let right_mid_y = (right_ymax + right_ymin) / 2.0\n        require left_mid_y >= right_mid_y",
        "cc_num",
        "3",
        "cc_num==3",
        "cc_num==3",
        "left_cc",
        "left_cc be Some(_)",
        "require left_cc be Some(_)",
        "right_cc",
        "right_cc be Some(_)",
        "require right_cc be Some(_)",
        "down_cc",
        "down_cc be Some(_)",
        "require down_cc be Some(_)",
        "down_cc",
        "down_cc!",
        "down_cc!.relative_bounding_box",
        "down_cc!.relative_bounding_box.ymin()",
        "0.4",
        "down_cc!.relative_bounding_box.ymin() <0.4",
        "down_cc!.relative_bounding_box.ymin() <0.4",
        "require down_cc!.relative_bounding_box.ymin() <0.4",
        "down_cc",
        "down_cc!",
        "down_cc!.angle_change",
        "let a = down_cc!.angle_change",
        "if cc_num==3:\n        require left_cc be Some(_)\n        require right_cc be Some(_)\n        require down_cc be Some(_)\n        require down_cc!.relative_bounding_box.ymin() <0.4\n        let a = down_cc!.angle_change",
        "OneVsAll::Yes",
        "OneVsAll::Yes",
        "let cc_num = major_concave_components.ilen()\n    let eff_holes = major_connected_component.eff_holes\n    \n    require eff_holes.matches[1] be None\n    let left_cc = two_match.matches[0]\n    let right_cc = two_match.matches[1]\n    let down_cc = two_match.matches[2]\n\n    require cc_num<=3\n\n    let lower_excess = major_connected_component.lower_mass - major_connected_component.upper_mass\n\n    require lower_excess > 10.0\n\n    if cc_num == 2:\n        require left_cc be Some(_)\n        require right_cc be Some(_)\n        let a = right_cc!.angle_change\n        require a>-180.0\n        // get the end line of the cc\n        let end_tan = left_cc!.end_tangent().angle(true)\n        // require end_tan < -200.0\n        let x = left_cc!.end_tangent().x\n        let y = left_cc!.end_tangent().y\n\n        let left_ymax = left_cc!.relative_bounding_box.ymax()\n        let left_ymin = left_cc!.relative_bounding_box.ymin()\n        let left_mid_y = (left_ymax + left_ymin) / 2.0\n\n        let right_ymax = right_cc!.relative_bounding_box.ymax()\n        let right_ymin = right_cc!.relative_bounding_box.ymin()\n        let right_mid_y = (right_ymax + right_ymin) / 2.0\n        require left_mid_y >= right_mid_y\n\n    // if cc_num == 2:\n    //     require major_connected_component.eff_holes.matches[0] be None\n    if cc_num==3:\n        require left_cc be Some(_)\n        require right_cc be Some(_)\n        require down_cc be Some(_)\n        require down_cc!.relative_bounding_box.ymin() <0.4\n        let a = down_cc!.angle_change\n    OneVsAll::Yes",
    ],
)
```
