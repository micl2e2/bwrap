# bwrap

[![crates][api_crate]][url_crate] [![unsafe][api_unsafe]][url_unsafe]
[![dep_status][api_depstatus]][url_depstatus] ![api_license][url_license]

[api_crate]: https://img.shields.io/crates/v/bwrap.svg
[url_crate]: https://crates.io/crates/bwrap
[api_license]: Crates.io
[url_license]: https://img.shields.io/crates/l/bwrap
[api_unsafe]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[url_unsafe]: https://github.com/rust-secure-code/safety-dance/
[api_depstatus]: https://deps.rs/repo/github/micl2e2/bwrap/status.svg
[url_depstatus]: https://deps.rs/repo/github/micl2e2/bwrap

# Table of Contents

- [About](#about)
- [Benchmark](#benchmark)
- [Features](#features)
- [Examples](#examples)
  - [Multilingual](#multilingual)
    - [English, Ukrainian, Greek, etc.](#english-ukrainian-greek-etc)
    - [Chinese, Japanese, Thai, etc.](#chinese-japanese-thai-etc)
  - [Text Appending](#text-appending)
    - [Indentation](#indentation)
    - [Trailing notation](#trailing_notation)
- [Contributing](#contributing)
- [License](#license)



# About

Bwrap is a fast, lightweight, embedded system-friendly library for
wrapping text. While bwrap offers great flexibility in wrapping text,
neither resource consumption nor performance compromises: 

1.  No heap allocation happens by default.

2.  The time/space complexity is *O(n)* by default, or *O(n(p+a))* if
    there is appending/prepending. (*n*, *p*, *a* is the number of
    input/prepending/appending bytes respectively) 

For the sake of readability, we **b**etter **wrap** our text. 



# Benchmark

Below are the performance comparisons among several text-wrapping
libraries in different dimensions:

**Time:**

<img
src="https://github.com/micl2e2/bench-wrap-libs/blob/master/examples/result-rtime.png"
width="600px"/>

**Memory:**

<img
src="https://github.com/micl2e2/bench-wrap-libs/blob/master/examples/result-mempeak.png"
width="600px"/>

Note:

1.  The benchmark is reproduciable, details about benchmark samples or
    methods are elaborated in
    [bench-wrap-libs](https://github.com/micl2e2/bench-wrap-libs). 

2.  The data above is obtained on an i5-3337u/8G machine and is for
    reference only. It is possible to have a slightly different result
    on a different machine or with different idle system resource. 


# Features

`use_std`: Use Rust standard library(libstd) for automatic memory management.


# Examples

Note that the following examples require `use_std` feature.

## Multilingual

bwrap suuport multiple languages, it categorizes languages into two
categories: **space-sensitive** and **space-insensitive**. The former
is for the languages that depend on ASCII SPACE to delimit words, such
as English, Ukrainian, Greek and so on. The latter is for the
languages that are space-insensitive, such as Chinese, Japanese, Thai
and so on. 


### English, Ukrainian, Greek, etc.

-   English

    Original:
    
    ```
    one two three four five six seven eight nine ten one two three four five six seven eight nine ten one two three four five six seven eight nine ten
    ```
    
    **Wrapped**:
    
    ```
    one two three four five six seven eight nine ten
    one two three four five six seven eight nine ten
    one two three four five six seven eight nine ten
    ```
    
    Source code:
    
    ```rust
    let line = "one two three four five six seven eight nine ten one two three four five six seven eight nine ten one two three four five six seven eight nine ten";
    println!("ORIGINAL:\n\n{}\n", line);
    println!("WRAPPED:\n\n{}", bwrap::wrap!(line, 50));
    ```

-   Ukrainian

    Original:
    
    ```
    один два три чотири п'ять шість сім вісім дев'ять десять один два три чотири п'ять шість сім вісім дев'ять десять один два три чотири п'ять шість сім вісім дев'ять десять
    ```
    
    **Wrapped**:
    
    ```
    один два три чотири п'ять шість сім вісім дев'ять десять
    один два три чотири п'ять шість сім вісім дев'ять десять
    один два три чотири п'ять шість сім вісім дев'ять десять
    ```
    
    Source code:
    
    ```rust
    let line = "один два три чотири п'ять шість сім вісім дев'ять десять один два три чотири п'ять шість сім вісім дев'ять десять один два три чотири п'ять шість сім вісім дев'ять десять";
    println!("ORIGINAL:\n\n{}\n", line);
    println!("WRAPPED:\n\n{}", bwrap::wrap!(line, 60));
    ```

-   Greek

    Original:
    
    ```
    ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα
    ```
    
    **Wrapped**:
    
    ```
    ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα
    ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα
    ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα
    ```
    
    Source code:
    
    ```rust
    let line = "ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα";
    println!("ORIGINAL:\n\n{}\n", line);
    println!("WRAPPED:\n\n{}", bwrap::wrap!(line, 51));
    ```


### Chinese, Japanese, Thai, etc.

-   Chinese

    Original:
    
    ```
    一二三四五六七八九十一二三四五六七八九十一二三四五六七八九十
    ```
    
    **Wrapped**:
    
    ```
    一二三四五六七八九十
    一二三四五六七八九十
    一二三四五六七八九十
    ```
    
    Source code:
    
    ```rust
    let line = "一二三四五六七八九十一二三四五六七八九十一二三四五六七八九十";
    println!("ORIGINAL:\n\n{}\n", line);
    println!("WRAPPED:\n\n{}", bwrap::wrap_maybrk!(line, 20));
    ```

-   Japanese

    Original:
    
    ```
    ありがとうございますありがとうございますありがとうございます
    ```
    
    **Wrapped**:
    
    ```
    ありがとうございます
    ありがとうございます
    ありがとうございます
    ```
    
    Source code:
    
    ```rust
    let line = "ありがとうございますありがとうございますありがとうございます";
    println!("ORIGINAL:\n\n{}\n", line);
    println!("WRAPPED:\n\n{}", bwrap::wrap_maybrk!(line, 20));
    ```

-   Thai

    Original:
    
    ```
    หนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบหนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบหนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบ
    ```
    
    **Wrapped**:
    
    ```
    หนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบ
    หนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบ
    หนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบ
    ```
    
    Source code:
    
    ```rust
    let line = "หนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบหนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบหนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบ";
    println!("ORIGINAL:\n\n{}\n", line);
    println!("WRAPPED:\n\n{}", bwrap::wrap_maybrk!(line, 25));
    ```



## Text Appending

bwrap can append or prepend whatever string to newly added newline
character. With this feature, one can effectively achieve indentation,
line trailing notation or similar. 



### Indentation

Original:

```
Here is our schedule:
- Do A, and do B, and do C, and do D, and do E, and do F
- Do G, and do H, and do I, and do J, and do K, and do L
```

**Wrapped**:

```
Here is our schedule:
- Do A, and do B, and do C, and do
  D, and do E, and do F
- Do G, and do H, and do I, and do
  J, and do K, and do L
```

Source code:

```rust
let line = "Here is our schedule:\n- Do A, and do B, and do C, and do D, and do E, and do F\n- Do G, and do H, and do I, and do J, and do K, and do L";
println!("ORIGINAL:\n\n{}\n", line);
println!("WRAPPED:\n\n{}", bwrap::wrap_nobrk!(line, 35, "  "));
```


### Trailing notation

Original:

```
VGhpcyBpcyBhIHNlY3JldCBtZXNzYWdlLCBwbGVhc2UgZGVsZXRlIGFmdGVyIHJlYWQK
```

**Wrapped**:

```
VGhpcyBpcy |
BhIHNlY3Jl |
dCBtZXNzYW |
dlLCBwbGVh |
c2UgZGVsZX |
RlIGFmdGVy |
IHJlYWQK  
```

Source code:

```rust
let line = "VGhpcyBpcyBhIHNlY3JldCBtZXNzYWdlLCBwbGVhc2UgZGVsZXRlIGFmdGVyIHJlYWQK";
println!("ORIGINAL:\n\n{}\n", line);
println!("WRAPPED:\n\n{}", bwrap::wrap_maybrk!(line, 10, " |"));
```


# Contributing

bwrap is still far from perfect, any form of contribution is welcomed!

This is not an exhaustive list:

- Ask question (label [![](https://img.shields.io/static/v1?label=&message=question&color=purple)](https://github.com/micl2e2/bwrap/issues/new))
- Bug Report (label [![](https://img.shields.io/static/v1?label=&message=bug&color=red)](https://github.com/micl2e2/bwrap/issues/new))
- Feature Request (label [![](https://img.shields.io/static/v1?label=&message=enhancement&color=lightblue)](https://github.com/micl2e2/bwrap/issues/new))
- Documentation Improvement (label [![](https://img.shields.io/static/v1?label=&message=enhancement&color=lightblue)](https://github.com/micl2e2/bwrap/issues/new))
- Code Contribution



# License

bwrap can be licensed under either [MIT
License](https://github.com/micl2e2/bwrap/blob/master/LICENSE-MIT) **or**
[GNU General Public License Version
3.0](https://github.com/micl2e2/bwrap/blob/master/LICENSE-GPL). The
choice is **entirely** up to you. 
