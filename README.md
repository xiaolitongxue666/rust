<br>

<img align="left" width="90" height="90" src="https://raw.githubusercontent.com/exercism/website-icons/main/tracks/rust.svg">
<p vertical-align="middle"><h1>Exercism Rust Track</h1></p>

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[![Discourse topics](https://img.shields.io/discourse/topics?color=8A08E6&label=Connect%20&labelColor=FFDF58&logo=Discourse&logoColor=8A08E6&server=https%3A%2F%2Fforum.exercism.org&style=social)](https://forum.exercism.org)
&nbsp;&nbsp;[![Exercism_III](https://img.shields.io/badge/PAUSED-C73D4E?labelColor=3D454D&label=Contributions)](https://exercism.org/blog/freeing-our-maintainers)
&nbsp;&nbsp;[![CI](https://github.com/xiaolitongxue666/rust/workflows/CI/badge.svg?branch=main)](https://github.com/xiaolitongxue666/rust/actions?query=workflow%3ACI+branch%3Amain)

> **个人练习仓库**：Fork 自 [exercism/rust](https://github.com/exercism/rust)，用于 Rust 学习与 Exercism 题目练习。

<br>

Hi. &nbsp;👋🏽 &nbsp;👋 &nbsp;**We are happy you are here.**&nbsp; 🎉&nbsp;🌟

<br>

**`exercism/rust`** is one of many programming language tracks on [exercism(dot)org][exercism-website].
This repo holds all the instructions, tests, code, & support files for Rust _exercises_ currently under development or implemented & available for students.

Some Exercism language tracks have a **syllabus** which is meant to teach the language step-by-step.
The Rust track's syllabus is a work in progress and it's not activated yet.
All exercises presented to students are **practice exercises**.
Students are expected to learn the language themselves, for example with the [official book][the-rust-programming-language], and practice with our exercises.

<br><br>

<div>
<span>
<img align="left" height="60" width="85" src="https://user-images.githubusercontent.com/5923094/204436863-2ebf34d1-4b16-486b-9e0a-add36f4c09c1.svg">
</span>
<span align="left">

🌟🌟&nbsp; Please take a moment to read our [Code of Conduct][exercism-code-of-conduct] &nbsp;🌟🌟<br>
It might also be helpful to look at [Being a Good Community Member][being-a-good-community-member] & [The words that we use][the-words-that-we-use].<br>
Some defined roles in our community: [Contributors][exercism-contributors] **|** [Mentors][exercism-mentors] **|** [Maintainers][exercism-track-maintainers] **|** [Admins][exercism-admins]

</span></div>

<br>
<img align="left" width="90" height="85" src="https://raw.githubusercontent.com/exercism/website-icons/main/exercises/health-statistics.svg">

We&nbsp;💛&nbsp;💙 &nbsp; our community.<br>
**`But our maintainers are not accepting community contributions at this time.`**<br>
Please read this [community blog post][freeing-maintainers] for details.

<br>
<img align="left" width="95" height="90" src="https://raw.githubusercontent.com/exercism/website-icons/main/exercises/boutique-suggestions.svg">

Here to suggest a new feature or new exercise?? **Hooray!** &nbsp;🎉 &nbsp;<br>
We'd love if you did that via our [Exercism Community Forum](https://forum.exercism.org/).<br>
Please read [Suggesting Exercise Improvements][suggesting-improvements] & [Chesterton's Fence][chestertons-fence].<br>
_Thoughtful suggestions will likely result faster & more enthusiastic responses from volunteers._

<br>
<img align="left" width="85" height="80" src="https://raw.githubusercontent.com/exercism/website-icons/main/exercises/word-search.svg">

✨&nbsp;🦄&nbsp; _**Want to jump directly into Exercism specifications & detail?**_<br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[Structure][exercism-track-structure] **|** [Tasks][exercism-tasks] **|** [Concepts][exercism-concepts] **|** [Concept Exercises][concept-exercises] **|** [Practice Exercises][practice-exercises] **|** [Presentation][exercise-presentation]<br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[Writing Style Guide][exercism-writing-style] **|** [Markdown Specification][exercism-markdown-specification] (_✨ version in [contributing][website-contributing-section] on exercism.org_)

<br>
<br>

## 练习顺序（学习参考）

按 [Exercism Rust Track](https://exercism.org/tracks/rust/exercises) 官方顺序，共 99 道 practice 题目（已排除 deprecated）：

| 序号 | 题目 | 难度 | 主要考点 |
| --- | --- | --- | --- |
| 1 | hello-world | Easy | TDD |
| 2 | reverse-string | Easy | 迭代器、字符串 |
| 3 | gigasecond | Easy | external-crates |
| 4 | clock | Medium | structs、derive、traits |
| 5 | anagram | Medium | 迭代器、lifetimes |
| 6 | space-age | Medium+ | From trait、traits |
| 7 | sublist | Medium+ | 泛型、enums |
| 8 | flower-field | Medium+ | 棋盘状态 |
| 9 | luhn | Medium+ | 高阶函数、迭代器 |
| 10 | armstrong-numbers | Easy | 数学 |
| 11 | bottle-song | Easy | - |
| 12 | difference-of-squares | Easy | map、fold、数学 |
| 13 | grains | Easy | panic |
| 14 | leap | Easy | 条件、取模 |
| 15 | nth-prime | Easy | 数学、素数 |
| 16 | prime-factors | Easy | 数学 |
| 17 | proverb | Easy | format |
| 18 | raindrops | Easy | 条件、字符串 |
| 19 | sum-of-multiples | Easy | 算法、借用 |
| 20 | bob | Easy | char、字符串 |
| 21 | high-scores | Easy | 迭代器、lifetimes |
| 22 | matching-brackets | Easy | 栈 |
| 23 | collatz-conjecture | Easy | Option、数学 |
| 24 | series | Easy | 字符串、切片 |
| 25 | kindergarten-garden | Easy | - |
| 26 | eliuds-eggs | Easy | 位运算 |
| 27 | acronym | Medium | filter、map、驼峰 |
| 28 | all-your-base | Medium | 进制转换、Result |
| 29 | allergies | Medium | 位运算、filter |
| 30 | alphametics | Medium | 组合、解析 |
| 31 | binary-search | Medium | Option、切片 |
| 32 | bowling | Medium | structs、Result |
| 33 | dot-dsl | Medium | builder、derive |
| 34 | etl | Easy | BTreeMap |
| 35 | grade-school | Medium | Entry API、HashMap |
| 36 | hamming | Medium | Option、zip |
| 37 | isbn-verifier | Medium | 字符串解析 |
| 38 | isogram | Medium | char、迭代器 |
| 39 | nucleotide-count | Medium | Entry API、Result |
| 40 | paasio | Medium | I/O、traits |
| 41 | palindrome-products | Medium | 数学、structs |
| 42 | pangram | Medium | HashSet、filter |
| 43 | pascals-triangle | Medium | 数学、Vec |
| 44 | perfect-numbers | Medium | 数学 |
| 45 | pig-latin | Medium | 字符串、正则 |
| 46 | queen-attack | Medium | structs、Result |
| 47 | rna-transcription | Medium | match、字符串 |
| 48 | run-length-encoding | Medium | 游程编码 |
| 49 | saddle-points | Medium | 迭代器、矩阵 |
| 50 | say | Medium | 数字转英文 |
| 51 | scrabble-score | Medium | HashMap、match |
| 52 | sieve | Medium | 埃氏筛 |
| 53 | simple-linked-list | Medium | Box、链表 |
| 54 | spiral-matrix | Medium | 矩阵、螺旋 |
| 55 | tournament | Medium | HashMap、排序 |
| 56 | triangle | Medium | structs |
| 57 | two-bucket | Medium | BFS/DFS |
| 58 | variable-length-quantity | Medium | 位运算、VLQ |
| 59 | robot-simulator | Medium+ | enums |
| 60 | robot-name | Medium | 随机、lifetimes |
| 61 | protein-translation | Medium+ | HashMap、Result |
| 62 | wordy | Medium | 解析、表达式 |
| 63 | custom-set | Medium | 泛型、集合 |
| 64 | accumulate | Medium | 高阶函数、FnMut |
| 65 | affine-cipher | Medium | 仿射密码 |
| 66 | atbash-cipher | Medium | 替换密码 |
| 67 | crypto-square | Medium | 密码方阵 |
| 68 | diamond | Medium | 字符串、格式 |
| 69 | largest-series-product | Medium | 滑动窗口、数学 |
| 70 | luhn-from | Medium | From trait |
| 71 | luhn-trait | Medium | trait、blanket impl |
| 72 | list-ops | Medium | 迭代器、泛型 |
| 73 | phone-number | Medium | 字符串解析、Option |
| 74 | rail-fence-cipher | Medium | 栅栏密码 |
| 75 | roman-numerals | Medium | From、Display |
| 76 | rotational-cipher | Medium | char、ASCII |
| 77 | simple-cipher | Medium | Vigenère 密码 |
| 78 | word-count | Medium | HashMap、Entry API |
| 79 | secret-handshake | Medium | 位运算 |
| 80 | knapsack | Medium | 动态规划 |
| 81 | yacht | Medium | 骰子、模式匹配 |
| 82 | matrix | Medium | 迭代器、切片 |
| 83 | fizzy | Medium+ | 泛型、impl Trait |
| 84 | pythagorean-triplet | Medium+ | 数学、三元组 |
| 85 | parallel-letter-frequency | Hard | 多线程 |
| 86 | macros | Hard | macro_rules! |
| 87 | pov | Hard | 树、图算法 |
| 88 | poker | Hard | 解析、枚举、比较 |
| 89 | forth | Hard | 栈、词法解析 |
| 90 | ocr-numbers | Hard | 3x4 网格、模式匹配 |
| 91 | react | Hard | 响应式、闭包 |
| 92 | circular-buffer | Hard | 环形缓冲区 |
| 93 | rectangles | Hard | ASCII 图形 |
| 94 | xorcism | Hard | XOR、生命周期 |
| 95 | book-store | Medium+ | 动态规划 |
| 96 | decimal | Medium+ | 任意精度、bigint |
| 97 | grep | Medium+ | 文件 I/O、anyhow |
| 98 | dominoes | Hard | 图论、回溯 |
| 99 | doubly-linked-list | Hard | unsafe、裸指针 |

**测试命令**：`cargo test -- --include-ignored`  
**特殊**：xorcism 需 `--features io`；doubly-linked-list 需 `--features advanced`

### 项目结构

| 路径 | 说明 |
| --- | --- |
| `hello-world/`、`reverse-string/` 等（99 道） | 个人实现的题目（`exercism download` 下载） |
| `exercises/practice/<slug>/` | 题目规范定义与参考实现 |
| `Install_Exercism_Tool.md` | Exercism CLI 安装与使用指南 |

<br>

## Exercism Rust Track License

This repository uses the [MIT License](/LICENSE).

[being-a-good-community-member]: https://github.com/exercism/docs/tree/main/community/good-member
[chestertons-fence]: https://github.com/exercism/docs/blob/main/community/good-member/chestertons-fence.md
[concept-exercises]: https://github.com/exercism/docs/blob/main/building/tracks/concept-exercises.md
[exercise-presentation]: https://github.com/exercism/docs/blob/main/building/tracks/presentation.md
[exercism-admins]: https://github.com/exercism/docs/blob/main/community/administrators.md
[exercism-code-of-conduct]: https://exercism.org/docs/using/legal/code-of-conduct
[exercism-concepts]: https://github.com/exercism/docs/blob/main/building/tracks/concepts.md
[exercism-contributors]: https://github.com/exercism/docs/blob/main/community/contributors.md
[exercism-markdown-specification]: https://github.com/exercism/docs/blob/main/building/markdown/markdown.md
[exercism-mentors]: https://github.com/exercism/docs/tree/main/mentoring
[exercism-tasks]: https://exercism.org/docs/building/product/tasks
[exercism-track-maintainers]: https://github.com/exercism/docs/blob/main/community/maintainers.md
[exercism-track-structure]: https://github.com/exercism/docs/tree/main/building/tracks
[exercism-website]: https://exercism.org/
[exercism-writing-style]: https://github.com/exercism/docs/blob/main/building/markdown/style-guide.md
[freeing-maintainers]: https://exercism.org/blog/freeing-our-maintainers
[practice-exercises]: https://github.com/exercism/docs/blob/main/building/tracks/practice-exercises.md
[suggesting-improvements]: https://github.com/exercism/docs/blob/main/community/good-member/suggesting-exercise-improvements.md
[the-words-that-we-use]: https://github.com/exercism/docs/blob/main/community/good-member/words.md
[website-contributing-section]: https://exercism.org/docs/building
[the-rust-programming-language]: https://doc.rust-lang.org/book/
