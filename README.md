<br>

<img align="left" width="90" height="90" src="https://raw.githubusercontent.com/exercism/website-icons/main/tracks/rust.svg">
<p vertical-align="middle"><h1>Exercism Rust Track</h1></p>

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;[![Discourse topics](https://img.shields.io/discourse/topics?color=8A08E6&label=Connect%20&labelColor=FFDF58&logo=Discourse&logoColor=8A08E6&server=https%3A%2F%2Fforum.exercism.org&style=social)](https://forum.exercism.org)
&nbsp;&nbsp;[![Exercism_III](https://img.shields.io/badge/PAUSED-C73D4E?labelColor=3D454D&label=Contributions)](https://exercism.org/blog/freeing-our-maintainers)
&nbsp;&nbsp;[![CI](https://github.com/xiaolitongxue666/rust/workflows/CI/badge.svg?branch=main)](https://github.com/xiaolitongxue666/rust/actions?query=workflow%3ACI+branch%3Amain)

> **个人练习仓库**：Fork 自 [exercism/rust](https://github.com/exercism/rust)，用于 Rust 学习与 Exercism 题目练习。根目录下的各题目文件夹为 `exercism download` 下载的实现目录。

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

## 24 题练习顺序（学习参考）

| 序号 | 题目 | 难度 | 主要考点 |
| --- | --- | --- | --- |
| 1 | rotational-cipher | Medium | char、ASCII、迭代器 |
| 2 | simple-cipher | Medium | Vigenère 密码、char |
| 3 | word-count | Medium | HashMap、Entry API、字符串解析 |
| 4 | secret-handshake | Medium | 位运算、二进制 |
| 5 | knapsack | Medium | 动态规划 |
| 6 | yacht | Medium | 骰子计分、模式匹配 |
| 7 | matrix | Medium | 迭代器、切片 |
| 8 | fizzy | Hard | 泛型、impl Trait、迭代器 |
| 9 | pythagorean-triplet | Medium | 数学、三元组 |
| 10 | parallel-letter-frequency | Hard | 多线程、并发 |
| 11 | macros | Hard | 声明宏 macro_rules! |
| 12 | pov | Hard | 树结构、图算法 |
| 13 | poker | Hard | 解析、枚举、比较 |
| 14 | forth | Hard | 栈、词法解析 |
| 15 | ocr-numbers | Hard | 模式匹配、3x4 网格 |
| 16 | react | Hard | 响应式、闭包、图 |
| 17 | circular-buffer | Hard | 环形缓冲区、泛型 |
| 18 | rectangles | Hard | ASCII 图形、算法 |
| 19 | xorcism | Hard | XOR、生命周期、迭代器 |
| 20 | book-store | Medium | 动态规划、组合优化 |
| 21 | decimal | Medium | 任意精度、traits |
| 22 | grep | Medium | 文件 I/O、anyhow |
| 23 | dominoes | Hard | 图论、回溯 |
| 24 | doubly-linked-list | Hard | unsafe、裸指针 |

**测试命令**：`cargo test -- --include-ignored`  
**特殊**：xorcism 需 `--features io`；doubly-linked-list 需 `--features advanced`

### 项目结构

| 路径 | 说明 |
| --- | --- |
| `rotational-cipher/`、`simple-cipher/` 等 | 个人实现的题目（`exercism download` 下载） |
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
