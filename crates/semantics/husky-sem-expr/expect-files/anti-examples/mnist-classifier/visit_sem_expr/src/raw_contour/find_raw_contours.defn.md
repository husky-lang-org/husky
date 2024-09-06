```rust
Some(
    [
        "[]",
        "let mut result: []RawContour = []",
        "BinaryGrid28::new_zeros",
        "BinaryGrid28::new_zeros()",
        "let mut boundary_unsearched = BinaryGrid28::new_zeros()",
        "1",
        "29",
        "i",
        "cc",
        "cc.mask",
        "i",
        "1",
        "i-1",
        "cc.mask[i-1]",
        "let r_ur = cc.mask[i-1]",
        "cc",
        "cc.mask",
        "i",
        "cc.mask[i]",
        "let r_dr = cc.mask[i]",
        "r_ur",
        "1",
        "r_ur << 1",
        "let r_ul = r_ur << 1",
        "r_dr",
        "1",
        "r_dr << 1",
        "let r_dl = r_dr << 1",
        "boundary_unsearched",
        "i",
        "boundary_unsearched[i]",
        "r_ur",
        "r_dr",
        "r_ur|r_dr",
        "r_ul",
        "r_ur|r_dr|r_ul",
        "r_dl",
        "r_ur|r_dr|r_ul|r_dl",
        "(r_ur|r_dr|r_ul|r_dl)",
        "r_ur",
        "r_dr",
        "r_ur&r_dr",
        "r_ul",
        "r_ur&r_dr&r_ul",
        "r_dl",
        "r_ur&r_dr&r_ul&r_dl",
        "(r_ur&r_dr&r_ul&r_dl)",
        "~(r_ur&r_dr&r_ul&r_dl)",
        "(~(r_ur&r_dr&r_ul&r_dl))",
        "(r_ur|r_dr|r_ul|r_dl) & (~(r_ur&r_dr&r_ul&r_dl))",
        "boundary_unsearched[i] = (r_ur|r_dr|r_ul|r_dl) & (~(r_ur&r_dr&r_ul&r_dl))",
        "boundary_unsearched[i] = (r_ur|r_dr|r_ul|r_dl) & (~(r_ur&r_dr&r_ul&r_dl))",
        "for 1 <= i <= 29:\n        let r_ur = cc.mask[i-1]\n        let r_dr = cc.mask[i]\n        let r_ul = r_ur << 1\n        let r_dl = r_dr << 1\n        boundary_unsearched[i] = (r_ur|r_dr|r_ul|r_dl) & (~(r_ur&r_dr&r_ul&r_dl))",
        "1",
        "29",
        "k",
        "boundary_unsearched",
        "k",
        "boundary_unsearched[k]",
        "boundary_unsearched[k]",
        "[]",
        "let mut contour: []Point2d = []",
        "k",
        "let mut i = k",
        "boundary_unsearched",
        "k",
        "boundary_unsearched[k]",
        "boundary_unsearched[k].ctz()",
        "let mut j = boundary_unsearched[k].ctz()",
        "cc",
        "cc.mask",
        "i",
        "1",
        "i-1",
        "cc.mask[i-1]",
        "let mut row_above = cc.mask[i-1]",
        "cc",
        "cc.mask",
        "i",
        "cc.mask[i]",
        "let mut row_below = cc.mask[i]",
        "get_inward_direction",
        "row_above",
        "row_below",
        "j",
        "get_inward_direction(row_above, row_below, j)",
        "let mut inward_direction = get_inward_direction(row_above, row_below, j)",
        "i",
        "let i0 = i",
        "j",
        "let j0 = j",
        "inward_direction",
        "let dir0 = inward_direction",
        "0",
        "let mut prev_angle_change1 = 0",
        "0",
        "let mut prev_angle_change2 = 0",
        "0",
        "let mut total_angle_change = 0",
        "1",
        "-1",
        "let mut prev_streak1 = -1",
        "1",
        "-1",
        "let mut prev_streak2 = -1",
        "1",
        "-1",
        "let mut current_streak = -1",
        "get_outward_direction",
        "row_above",
        "row_below",
        "j",
        "inward_direction",
        "get_outward_direction(\n                    row_above,\n                    row_below,\n                    j,\n                    inward_direction\n                )",
        "let outward_direction = get_outward_direction(\n                    row_above,\n                    row_below,\n                    j,\n                    inward_direction\n                )",
        "get_angle_change",
        "inward_direction",
        "outward_direction",
        "get_angle_change(inward_direction, outward_direction)",
        "let angle_change = get_angle_change(inward_direction, outward_direction)",
        "boundary_unsearched",
        "i",
        "boundary_unsearched[i]",
        "boundary_unsearched",
        "i",
        "boundary_unsearched[i]",
        "1r32",
        "j",
        "1r32 << j",
        "(1r32 << j)",
        "~(1r32 << j)",
        "(~(1r32 << j))",
        "boundary_unsearched[i] & (~(1r32 << j))",
        "boundary_unsearched[i] = boundary_unsearched[i] & (~(1r32 << j))",
        "boundary_unsearched[i] = boundary_unsearched[i] & (~(1r32 << j))",
        "angle_change",
        "angle_change",
        "prev_angle_change1",
        "1",
        "-1",
        "prev_angle_change1 == -1",
        "prev_angle_change2",
        "1",
        "-1",
        "prev_angle_change2 == -1",
        "prev_angle_change1 == -1\n                            and prev_angle_change2 == -1",
        "current_streak",
        "1",
        "current_streak == 1",
        "prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1",
        "prev_streak1",
        "1",
        "-1",
        "prev_streak1 != -1",
        "prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1",
        "prev_streak2",
        "1",
        "prev_streak2 == 1",
        "prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1\n                            and prev_streak2 == 1",
        "prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1\n                            and prev_streak2 == 1",
        "contour",
        "contour.last()",
        "contour.last()!",
        "get_concave_middle_point",
        "contour",
        "get_concave_middle_point(contour)",
        "contour.last()! = get_concave_middle_point(contour)",
        "contour.last()! = get_concave_middle_point(contour)",
        "contour",
        "Point2d::from_i_shift28",
        "i",
        "j",
        "Point2d::from_i_shift28(i, j)",
        "contour.push(Point2d::from_i_shift28(i, j))",
        "contour.push(Point2d::from_i_shift28(i, j))",
        "prev_streak2",
        "1",
        "-1",
        "prev_streak2 = -1",
        "prev_streak2 = -1",
        "prev_streak1",
        "1",
        "-1",
        "prev_streak1 = -1",
        "prev_streak1 = -1",
        "prev_angle_change1",
        "1",
        "-1",
        "prev_angle_change1 == -1",
        "prev_streak1",
        "0",
        "prev_streak1 > 0",
        "prev_angle_change1 == -1\n                            and prev_streak1 > 0",
        "prev_streak1",
        "1",
        "prev_streak1 == 1",
        "prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and prev_streak1 == 1",
        "prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and prev_streak1 == 1",
        "contour",
        "contour.last()",
        "contour.last()!",
        "Point2d::from_i_shift28",
        "i",
        "j",
        "Point2d::from_i_shift28(i, j)",
        "contour.last()! = Point2d::from_i_shift28(i, j)",
        "contour.last()! = Point2d::from_i_shift28(i, j)",
        "prev_streak2",
        "prev_streak1",
        "prev_streak2 = prev_streak1",
        "prev_streak2 = prev_streak1",
        "prev_streak1",
        "current_streak",
        "prev_streak1 = current_streak",
        "prev_streak1 = current_streak",
        "prev_angle_change1",
        "1",
        "-1",
        "prev_angle_change1 == -1",
        "prev_streak1",
        "0",
        "prev_streak1 > 0",
        "prev_angle_change1 == -1\n                            and prev_streak1 > 0",
        "current_streak",
        "1",
        "current_streak == 1",
        "prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1",
        "prev_streak1",
        "1",
        "prev_streak1 > 1",
        "prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1\n                            and prev_streak1 > 1",
        "prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1\n                            and prev_streak1 > 1",
        "contour",
        "contour.last()",
        "contour.last()!",
        "Point2d::from_i_shift28",
        "i",
        "j",
        "Point2d::from_i_shift28(i, j)",
        "contour.last()! = Point2d::from_i_shift28(i, j)",
        "contour.last()! = Point2d::from_i_shift28(i, j)",
        "prev_streak2",
        "1",
        "-1",
        "prev_streak2 = -1",
        "prev_streak2 = -1",
        "prev_streak1",
        "1",
        "-1",
        "prev_streak1 = -1",
        "prev_streak1 = -1",
        "contour",
        "Point2d::from_i_shift28",
        "i",
        "j",
        "Point2d::from_i_shift28(i, j)",
        "contour.push(Point2d::from_i_shift28(i, j))",
        "contour.push(Point2d::from_i_shift28(i, j))",
        "prev_streak2",
        "prev_streak1",
        "prev_streak2 = prev_streak1",
        "prev_streak2 = prev_streak1",
        "prev_streak1",
        "current_streak",
        "prev_streak1 = current_streak",
        "prev_streak1 = current_streak",
        "if prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1\n                            and prev_streak2 == 1:\n                        contour.last()! = get_concave_middle_point(contour)\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and prev_streak1 == 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1\n                            and prev_streak1 > 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    else:\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak",
        "current_streak",
        "0",
        "current_streak = 0",
        "current_streak = 0",
        "prev_angle_change2",
        "prev_angle_change1",
        "prev_angle_change2 = prev_angle_change1",
        "prev_angle_change2 = prev_angle_change1",
        "prev_angle_change1",
        "angle_change",
        "prev_angle_change1 = angle_change",
        "prev_angle_change1 = angle_change",
        "if angle_change:\n                    // update contour and previous streaks\n                    if prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1\n                            and prev_streak2 == 1:\n                        contour.last()! = get_concave_middle_point(contour)\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and prev_streak1 == 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1\n                            and prev_streak1 > 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    else:\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    // reset current streak\n                    current_streak = 0\n                    // update previous angle change\n                    prev_angle_change2 = prev_angle_change1\n                    prev_angle_change1 = angle_change",
        "outward_direction",
        "i",
        "i",
        "1",
        "i - 1",
        "i = i - 1",
        "i = i - 1",
        "row_below",
        "row_above",
        "row_below = row_above",
        "row_below = row_above",
        "row_above",
        "cc",
        "cc.mask",
        "i",
        "1",
        "i-1",
        "cc.mask[i-1]",
        "row_above = cc.mask[i-1]",
        "row_above = cc.mask[i-1]",
        "i",
        "i",
        "1",
        "i + 1",
        "i = i + 1",
        "i = i + 1",
        "row_above",
        "row_below",
        "row_above = row_below",
        "row_above = row_below",
        "row_below",
        "cc",
        "cc.mask",
        "i",
        "cc.mask[i]",
        "row_below = cc.mask[i]",
        "row_below = cc.mask[i]",
        "j",
        "j",
        "1",
        "j + 1",
        "j = j + 1",
        "j = j + 1",
        "j",
        "j",
        "1",
        "j - 1",
        "j = j - 1",
        "j = j - 1",
        "match outward_direction with\n                | Direction::Up =>\n                    i = i - 1 \n                    row_below = row_above\n                    row_above = cc.mask[i-1]\n                | Direction::Down =>\n                    i = i + 1 \n                    row_above = row_below\n                    row_below = cc.mask[i]\n                | Direction::Left => j = j + 1\n                | Direction::Right => j = j - 1",
        "inward_direction",
        "outward_direction",
        "inward_direction = outward_direction",
        "inward_direction = outward_direction",
        "current_streak",
        "1",
        "-1",
        "current_streak != -1",
        "current_streak != -1",
        "current_streak",
        "current_streak++",
        "current_streak++",
        "if current_streak != -1:\n                    current_streak++",
        "i",
        "i0",
        "i == i0",
        "j",
        "j0",
        "j == j0",
        "i == i0 and j == j0",
        "inward_direction",
        "dir0",
        "inward_direction == dir0",
        "i == i0 and j == j0 and inward_direction == dir0",
        "(i == i0 and j == j0 and inward_direction == dir0)",
        "!(i == i0 and j == j0 and inward_direction == dir0)",
        "!(i == i0 and j == j0 and inward_direction == dir0)",
        "do while !(i == i0 and j == j0 and inward_direction == dir0):\n                let outward_direction = get_outward_direction(\n                    row_above,\n                    row_below,\n                    j,\n                    inward_direction\n                )\n                let angle_change = get_angle_change(inward_direction, outward_direction)\n                // update boundary_unsearched\n                boundary_unsearched[i] = boundary_unsearched[i] & (~(1r32 << j))\n                if angle_change:\n                    // update contour and previous streaks\n                    if prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1\n                            and prev_streak2 == 1:\n                        contour.last()! = get_concave_middle_point(contour)\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and prev_streak1 == 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1\n                            and prev_streak1 > 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    else:\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    // reset current streak\n                    current_streak = 0\n                    // update previous angle change\n                    prev_angle_change2 = prev_angle_change1\n                    prev_angle_change1 = angle_change\n                // move\n                // update i, j, row below and above\n                match outward_direction with\n                | Direction::Up =>\n                    i = i - 1 \n                    row_below = row_above\n                    row_above = cc.mask[i-1]\n                | Direction::Down =>\n                    i = i + 1 \n                    row_above = row_below\n                    row_below = cc.mask[i]\n                | Direction::Left => j = j + 1\n                | Direction::Right => j = j - 1\n                // update inward_direction\n                inward_direction = outward_direction\n                // update streak\n                if current_streak != -1:\n                    current_streak++",
        "prev_angle_change1",
        "1",
        "-1",
        "prev_angle_change1 == -1",
        "current_streak",
        "1",
        "current_streak == 1",
        "prev_angle_change1 == -1 and current_streak == 1",
        "prev_streak1",
        "0",
        "prev_streak1 > 0",
        "prev_angle_change1 == -1 and current_streak == 1 and prev_streak1 > 0",
        "prev_angle_change1 == -1 and current_streak == 1 and prev_streak1 > 0",
        "contour",
        "contour.pop()",
        "contour.pop()",
        "if prev_angle_change1 == -1 and current_streak == 1 and prev_streak1 > 0:\n                contour.pop()",
        "result",
        "RawContour",
        "cc",
        "contour",
        "RawContour(cc, contour)",
        "result.push(RawContour(cc, contour))",
        "result.push(RawContour(cc, contour))",
        "while boundary_unsearched[k]:\n            let mut contour: []Point2d = []\n            let mut i = k\n            let mut j = boundary_unsearched[k].ctz()\n            // prepare rows\n            let mut row_above = cc.mask[i-1]\n            let mut row_below = cc.mask[i]\n            // prepare pixel_pairs and initial inward direction\n            let mut inward_direction = get_inward_direction(row_above, row_below, j)\n            // store initial position and direction\n            let i0 = i\n            let j0 = j\n            let dir0 = inward_direction\n            let mut prev_angle_change1 = 0\n            let mut prev_angle_change2 = 0\n            let mut total_angle_change = 0\n            // prepare streaks (raw line segment lengths)\n            // -1 means invalid\n            let mut prev_streak1 = -1\n            let mut prev_streak2 = -1\n            let mut current_streak = -1\n            // loop in the geometric sense!\n            do while !(i == i0 and j == j0 and inward_direction == dir0):\n                let outward_direction = get_outward_direction(\n                    row_above,\n                    row_below,\n                    j,\n                    inward_direction\n                )\n                let angle_change = get_angle_change(inward_direction, outward_direction)\n                // update boundary_unsearched\n                boundary_unsearched[i] = boundary_unsearched[i] & (~(1r32 << j))\n                if angle_change:\n                    // update contour and previous streaks\n                    if prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1\n                            and prev_streak2 == 1:\n                        contour.last()! = get_concave_middle_point(contour)\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and prev_streak1 == 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1\n                            and prev_streak1 > 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    else:\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    // reset current streak\n                    current_streak = 0\n                    // update previous angle change\n                    prev_angle_change2 = prev_angle_change1\n                    prev_angle_change1 = angle_change\n                // move\n                // update i, j, row below and above\n                match outward_direction with\n                | Direction::Up =>\n                    i = i - 1 \n                    row_below = row_above\n                    row_above = cc.mask[i-1]\n                | Direction::Down =>\n                    i = i + 1 \n                    row_above = row_below\n                    row_below = cc.mask[i]\n                | Direction::Left => j = j + 1\n                | Direction::Right => j = j - 1\n                // update inward_direction\n                inward_direction = outward_direction\n                // update streak\n                if current_streak != -1:\n                    current_streak++\n            if prev_angle_change1 == -1 and current_streak == 1 and prev_streak1 > 0:\n                contour.pop();\n            result.push(RawContour(cc, contour))",
        "for 1 <= k <= 29:\n        while boundary_unsearched[k]:\n            let mut contour: []Point2d = []\n            let mut i = k\n            let mut j = boundary_unsearched[k].ctz()\n            // prepare rows\n            let mut row_above = cc.mask[i-1]\n            let mut row_below = cc.mask[i]\n            // prepare pixel_pairs and initial inward direction\n            let mut inward_direction = get_inward_direction(row_above, row_below, j)\n            // store initial position and direction\n            let i0 = i\n            let j0 = j\n            let dir0 = inward_direction\n            let mut prev_angle_change1 = 0\n            let mut prev_angle_change2 = 0\n            let mut total_angle_change = 0\n            // prepare streaks (raw line segment lengths)\n            // -1 means invalid\n            let mut prev_streak1 = -1\n            let mut prev_streak2 = -1\n            let mut current_streak = -1\n            // loop in the geometric sense!\n            do while !(i == i0 and j == j0 and inward_direction == dir0):\n                let outward_direction = get_outward_direction(\n                    row_above,\n                    row_below,\n                    j,\n                    inward_direction\n                )\n                let angle_change = get_angle_change(inward_direction, outward_direction)\n                // update boundary_unsearched\n                boundary_unsearched[i] = boundary_unsearched[i] & (~(1r32 << j))\n                if angle_change:\n                    // update contour and previous streaks\n                    if prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1\n                            and prev_streak2 == 1:\n                        contour.last()! = get_concave_middle_point(contour)\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and prev_streak1 == 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1\n                            and prev_streak1 > 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    else:\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    // reset current streak\n                    current_streak = 0\n                    // update previous angle change\n                    prev_angle_change2 = prev_angle_change1\n                    prev_angle_change1 = angle_change\n                // move\n                // update i, j, row below and above\n                match outward_direction with\n                | Direction::Up =>\n                    i = i - 1 \n                    row_below = row_above\n                    row_above = cc.mask[i-1]\n                | Direction::Down =>\n                    i = i + 1 \n                    row_above = row_below\n                    row_below = cc.mask[i]\n                | Direction::Left => j = j + 1\n                | Direction::Right => j = j - 1\n                // update inward_direction\n                inward_direction = outward_direction\n                // update streak\n                if current_streak != -1:\n                    current_streak++\n            if prev_angle_change1 == -1 and current_streak == 1 and prev_streak1 > 0:\n                contour.pop();\n            result.push(RawContour(cc, contour))",
        "result",
        "return result",
        "let mut result: []RawContour = []\n    let mut boundary_unsearched = BinaryGrid28::new_zeros()\n    for 1 <= i <= 29:\n        let r_ur = cc.mask[i-1]\n        let r_dr = cc.mask[i]\n        let r_ul = r_ur << 1\n        let r_dl = r_dr << 1\n        boundary_unsearched[i] = (r_ur|r_dr|r_ul|r_dl) & (~(r_ur&r_dr&r_ul&r_dl))\n    for 1 <= k <= 29:\n        while boundary_unsearched[k]:\n            let mut contour: []Point2d = []\n            let mut i = k\n            let mut j = boundary_unsearched[k].ctz()\n            // prepare rows\n            let mut row_above = cc.mask[i-1]\n            let mut row_below = cc.mask[i]\n            // prepare pixel_pairs and initial inward direction\n            let mut inward_direction = get_inward_direction(row_above, row_below, j)\n            // store initial position and direction\n            let i0 = i\n            let j0 = j\n            let dir0 = inward_direction\n            let mut prev_angle_change1 = 0\n            let mut prev_angle_change2 = 0\n            let mut total_angle_change = 0\n            // prepare streaks (raw line segment lengths)\n            // -1 means invalid\n            let mut prev_streak1 = -1\n            let mut prev_streak2 = -1\n            let mut current_streak = -1\n            // loop in the geometric sense!\n            do while !(i == i0 and j == j0 and inward_direction == dir0):\n                let outward_direction = get_outward_direction(\n                    row_above,\n                    row_below,\n                    j,\n                    inward_direction\n                )\n                let angle_change = get_angle_change(inward_direction, outward_direction)\n                // update boundary_unsearched\n                boundary_unsearched[i] = boundary_unsearched[i] & (~(1r32 << j))\n                if angle_change:\n                    // update contour and previous streaks\n                    if prev_angle_change1 == -1\n                            and prev_angle_change2 == -1\n                            and current_streak == 1\n                            and prev_streak1 != -1\n                            and prev_streak2 == 1:\n                        contour.last()! = get_concave_middle_point(contour)\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and prev_streak1 == 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    elif prev_angle_change1 == -1\n                            and prev_streak1 > 0\n                            and current_streak == 1\n                            and prev_streak1 > 1:\n                        contour.last()! = Point2d::from_i_shift28(i, j)\n                        prev_streak2 = -1\n                        prev_streak1 = -1\n                    else:\n                        contour.push(Point2d::from_i_shift28(i, j))\n                        prev_streak2 = prev_streak1\n                        prev_streak1 = current_streak\n                    // reset current streak\n                    current_streak = 0\n                    // update previous angle change\n                    prev_angle_change2 = prev_angle_change1\n                    prev_angle_change1 = angle_change\n                // move\n                // update i, j, row below and above\n                match outward_direction with\n                | Direction::Up =>\n                    i = i - 1 \n                    row_below = row_above\n                    row_above = cc.mask[i-1]\n                | Direction::Down =>\n                    i = i + 1 \n                    row_above = row_below\n                    row_below = cc.mask[i]\n                | Direction::Left => j = j + 1\n                | Direction::Right => j = j - 1\n                // update inward_direction\n                inward_direction = outward_direction\n                // update streak\n                if current_streak != -1:\n                    current_streak++\n            if prev_angle_change1 == -1 and current_streak == 1 and prev_streak1 > 0:\n                contour.pop();\n            result.push(RawContour(cc, contour))\n    return result",
    ],
)
```