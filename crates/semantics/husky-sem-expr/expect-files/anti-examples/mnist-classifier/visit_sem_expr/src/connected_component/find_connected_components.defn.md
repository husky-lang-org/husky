```rust
Some(
    [
        "[]",
        "let mut result: []ConnectedComponent = []",
        "img",
        "img.clone()",
        "let mut unsearched = img.clone()",
        "30",
        "j",
        "unsearched",
        "j",
        "unsearched[j]",
        "unsearched[j]",
        "unsearched",
        "j",
        "unsearched[j]",
        "let a = unsearched[j]",
        "a",
        "a.ctz()",
        "let shift = a.ctz()",
        "BinaryImage28::new_zeros",
        "BinaryImage28::new_zeros()",
        "let mut mask = BinaryImage28::new_zeros()",
        "mask",
        "j",
        "mask[j]",
        "horizontal_extend",
        "a",
        "1r32",
        "shift",
        "1r32 << shift",
        "horizontal_extend(a, 1r32 << shift)",
        "mask[j] = horizontal_extend(a, 1r32 << shift)",
        "mask[j] = horizontal_extend(a, 1r32 << shift)",
        "false",
        "let mut flag = false",
        "flag",
        "!flag",
        "!flag",
        "flag",
        "true",
        "flag = true",
        "flag = true",
        "j",
        "let mut i = j",
        "30",
        "1",
        "30 - 1",
        "mask",
        "i",
        "1",
        "i + 1",
        "mask[i + 1]",
        "let old_row = mask[i + 1]",
        "old_row",
        "horizontal_extend",
        "img",
        "i",
        "1",
        "i + 1",
        "img[i + 1]",
        "mask",
        "i",
        "mask[i]",
        "horizontal_extend(img[i + 1], mask[i])",
        "old_row | horizontal_extend(img[i + 1], mask[i])",
        "let new_row = old_row | horizontal_extend(img[i + 1], mask[i])",
        "new_row",
        "!new_row",
        "!new_row",
        "break",
        "if !new_row:\n                        break",
        "old_row",
        "new_row",
        "old_row != new_row",
        "old_row != new_row",
        "flag",
        "false",
        "flag = false",
        "flag = false",
        "mask",
        "i",
        "1",
        "i + 1",
        "mask[i + 1]",
        "new_row",
        "mask[i + 1] = new_row",
        "mask[i + 1] = new_row",
        "if old_row != new_row :\n                        flag = false\n                        mask[i + 1] = new_row",
        "forext i < 30 - 1:\n                    let old_row = mask[i + 1]\n                    let new_row = old_row | horizontal_extend(img[i + 1], mask[i])\n                    if !new_row:\n                        break\n                    if old_row != new_row :\n                        flag = false\n                        mask[i + 1] = new_row",
        "j",
        "mask",
        "i",
        "mask[i]",
        "let old_row = mask[i]",
        "old_row",
        "horizontal_extend",
        "img",
        "i",
        "img[i]",
        "mask",
        "i",
        "1",
        "i + 1",
        "mask[i + 1]",
        "horizontal_extend(img[i], mask[i + 1])",
        "old_row | horizontal_extend(img[i], mask[i + 1])",
        "let new_row = old_row | horizontal_extend(img[i], mask[i + 1])",
        "old_row",
        "new_row",
        "old_row != new_row",
        "old_row != new_row",
        "flag",
        "false",
        "flag = false",
        "flag = false",
        "mask",
        "i",
        "mask[i]",
        "new_row",
        "mask[i] = new_row",
        "mask[i] = new_row",
        "if old_row != new_row:\n                        flag = false\n                        mask[i] = new_row",
        "forext i >= j:\n                    let old_row = mask[i]\n                    let new_row = old_row | horizontal_extend(img[i], mask[i + 1])\n                    if old_row != new_row:\n                        flag = false\n                        mask[i] = new_row",
        "while !flag:\n                flag = true\n                let mut i = j\n                forext i < 30 - 1:\n                    let old_row = mask[i + 1]\n                    let new_row = old_row | horizontal_extend(img[i + 1], mask[i])\n                    if !new_row:\n                        break\n                    if old_row != new_row :\n                        flag = false\n                        mask[i + 1] = new_row\n                forext i >= j:\n                    let old_row = mask[i]\n                    let new_row = old_row | horizontal_extend(img[i], mask[i + 1])\n                    if old_row != new_row:\n                        flag = false\n                        mask[i] = new_row",
        "j",
        "30",
        "k",
        "unsearched",
        "k",
        "unsearched[k]",
        "mask",
        "k",
        "mask[k]",
        "~mask[k]",
        "(~mask[k])",
        "unsearched[k] &= (~mask[k])",
        "unsearched[k] &= (~mask[k])",
        "for j <= k < 30:\n                unsearched[k] &= (~mask[k])",
        "result",
        "ConnectedComponent",
        "mask",
        "ConnectedComponent(mask)",
        "result.push(ConnectedComponent(mask))",
        "result.push(ConnectedComponent(mask))",
        "while unsearched[j]:\n            let a = unsearched[j]\n            let shift = a.ctz()\n            let mut mask = BinaryImage28::new_zeros()\n            mask[j] = horizontal_extend(a, 1r32 << shift)\n            let mut flag = false\n            while !flag:\n                flag = true\n                let mut i = j\n                forext i < 30 - 1:\n                    let old_row = mask[i + 1]\n                    let new_row = old_row | horizontal_extend(img[i + 1], mask[i])\n                    if !new_row:\n                        break\n                    if old_row != new_row :\n                        flag = false\n                        mask[i + 1] = new_row\n                forext i >= j:\n                    let old_row = mask[i]\n                    let new_row = old_row | horizontal_extend(img[i], mask[i + 1])\n                    if old_row != new_row:\n                        flag = false\n                        mask[i] = new_row\n            for j <= k < 30:\n                unsearched[k] &= (~mask[k])\n            result.push(ConnectedComponent(mask))",
        "for j < 30:\n        while unsearched[j]:\n            let a = unsearched[j]\n            let shift = a.ctz()\n            let mut mask = BinaryImage28::new_zeros()\n            mask[j] = horizontal_extend(a, 1r32 << shift)\n            let mut flag = false\n            while !flag:\n                flag = true\n                let mut i = j\n                forext i < 30 - 1:\n                    let old_row = mask[i + 1]\n                    let new_row = old_row | horizontal_extend(img[i + 1], mask[i])\n                    if !new_row:\n                        break\n                    if old_row != new_row :\n                        flag = false\n                        mask[i + 1] = new_row\n                forext i >= j:\n                    let old_row = mask[i]\n                    let new_row = old_row | horizontal_extend(img[i], mask[i + 1])\n                    if old_row != new_row:\n                        flag = false\n                        mask[i] = new_row\n            for j <= k < 30:\n                unsearched[k] &= (~mask[k])\n            result.push(ConnectedComponent(mask))",
        "result",
        "return result",
        "let mut result: []ConnectedComponent = []\n    let mut unsearched = img.clone()\n    for j < 30:\n        while unsearched[j]:\n            let a = unsearched[j]\n            let shift = a.ctz()\n            let mut mask = BinaryImage28::new_zeros()\n            mask[j] = horizontal_extend(a, 1r32 << shift)\n            let mut flag = false\n            while !flag:\n                flag = true\n                let mut i = j\n                forext i < 30 - 1:\n                    let old_row = mask[i + 1]\n                    let new_row = old_row | horizontal_extend(img[i + 1], mask[i])\n                    if !new_row:\n                        break\n                    if old_row != new_row :\n                        flag = false\n                        mask[i + 1] = new_row\n                forext i >= j:\n                    let old_row = mask[i]\n                    let new_row = old_row | horizontal_extend(img[i], mask[i + 1])\n                    if old_row != new_row:\n                        flag = false\n                        mask[i] = new_row\n            for j <= k < 30:\n                unsearched[k] &= (~mask[k])\n            result.push(ConnectedComponent(mask))\n    return result",
    ],
)
```