
# Table of Contents

-   [About](#org8903a3c)
-   [Benchmark](#org58b3687)
-   [Examples](#orgac3f2b8)
    -   [Multiple languages](#org9d259cf)
        -   [English, Ukrainian, Greek, etc.](#orgcd80dbd)
        -   [Chinese, Japanese, Thai, etc.](#org2ce2185)
    -   [Append/prepend](#org869ba6e)
        -   [Indentation](#org3248f83)
        -   [Trailing notation](#orgad88a98)
-   [License](#org2ca36a1)



<a id="org8903a3c"></a>

# About

Bwrap is a fast, lightweight, embedded environment-friendly library
for wrapping text. While Bwrap offers great flexibility in wrapping
text, neither resource consumption nor performance compromises: 

1.  No heap allocation happens by default.

2.  The time/space complexity is *O(n)* by default, or *O(n(p+a))* if
    there is appending/prepending. (*n*, *p*, *a* is the number of
    input bytes, prepended bytes, and appended bytes respectively)

For the sake of readability, we (**b**)etter **wrap** our text.


<a id="org58b3687"></a>

# Benchmark

Below are the performance comparisons among several text-wrapping
libraries in different dimensions:

**Time elapsed:**

<img src="result-rtime.png" width="500"/>

**Memory usage:**

<img src="result-mempeak.png" width="500"/>

Note:

1.  Details about benchmark samples or methods are elaborated in
    [bench-wrap-libs](https://github.com/imichael2e2/bench-wrap-libs).

2.  The benchmark results above are obtained on an i7u/16G machine and
    are for reference only. Different machines or idle system resource
    might generate different results.


<a id="orgac3f2b8"></a>

# Examples


<a id="org9d259cf"></a>

## Multiple languages

Bwrap suuport multiple languages, it categorizes languages into two
categories: **space-sensitive** and **space-insensitive**. The former is
for the languages that depend on ASCII SPACE to delimit words, such as
English, Ukrainian, Greek and so on. The latter is for the languages
that are space-insensitive, such as Chinese, Japanese, Thai and so on.


<a id="orgcd80dbd"></a>

### English, Ukrainian, Greek, etc.

-   English

    Original:
    
        one two three four five six seven eight nine ten one two three four five six seven eight nine ten one two three four five six seven eight nine ten
    
    **Wrapped**:
    
        one two three four five six seven eight nine ten
        one two three four five six seven eight nine ten
        one two three four five six seven eight nine ten
    
    Source code:
    
        let line = "one two three four five six seven eight nine ten one two three four five six seven eight nine ten one two three four five six seven eight nine ten";
        println!("ORIGINAL:\n\n{}\n", line);
        println!("WRAPPED:\n\n{}", bwrap::wrap!(line, 50));

-   Ukrainian

    Original:
    
        один два три чотири п'ять шість сім вісім дев'ять десять один два три чотири п'ять шість сім вісім дев'ять десять один два три чотири п'ять шість сім вісім дев'ять десять
    
    **Wrapped**:
    
        один два три чотири п'ять шість сім вісім дев'ять десять
        один два три чотири п'ять шість сім вісім дев'ять десять
        один два три чотири п'ять шість сім вісім дев'ять десять
    
    Source code:
    
        let line = "один два три чотири п'ять шість сім вісім дев'ять десять один два три чотири п'ять шість сім вісім дев'ять десять один два три чотири п'ять шість сім вісім дев'ять десять";
        println!("ORIGINAL:\n\n{}\n", line);
        println!("WRAPPED:\n\n{}", bwrap::wrap!(line, 60));

-   Greek

    Original:
    
        ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα
    
    **Wrapped**:
    
        ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα
        ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα
        ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα
    
    Source code:
    
        let line = "ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα ένα δύο τρία τέσσερα πέντε έξι επτά οκτώ εννέα δέκα";
        println!("ORIGINAL:\n\n{}\n", line);
        println!("WRAPPED:\n\n{}", bwrap::wrap!(line, 51));


<a id="org2ce2185"></a>

### Chinese, Japanese, Thai, etc.

-   Chinese

    Original:
    
        一二三四五六七八九十一二三四五六七八九十一二三四五六七八九十
    
    **Wrapped**:
    
        一二三四五六七八九十
        一二三四五六七八九十
        一二三四五六七八九十
    
    Source code: 
    
        let line = "一二三四五六七八九十一二三四五六七八九十一二三四五六七八九十";
        println!("ORIGINAL:\n\n{}\n", line);
        let mut w = bwrap::EasyWrapper::new(line, 20).unwrap();
        let wrapped = w.wrap_use_style(bwrap::WrapStyle::MayBrk(None, None)).unwrap();
        println!("WRAPPED:\n\n{}", wrapped);

-   Japanese

    Original:
    
        ありがとうございますありがとうございますありがとうございます
    
    **Wrapped**:
    
        ありがとうございます
        ありがとうございます
        ありがとうございます
    
    Source code:
    
        let line = "ありがとうございますありがとうございますありがとうございます";
        println!("ORIGINAL:\n\n{}\n", line);
        let mut w = bwrap::EasyWrapper::new(line, 10).unwrap();
        let wrapped = w.wrap_use_style(bwrap::WrapStyle::MayBrk(None, None)).unwrap();
        println!("WRAPPED:\n\n{}", wrapped);

-   Thai

    Original:
    
        หนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบหนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบหนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบ
    
    **Wrapped**:
    
        หนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบ
        หนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบ
        หนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบ
    
    Source code:
    
        let line = "หนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบหนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบหนึ่งสองสามสี่ห้าหกเจ็ดแปดเก้าสิบ";
        println!("ORIGINAL:\n\n{}\n", line);
        let mut w = bwrap::EasyWrapper::new(line, 25).unwrap();
        let wrapped = w.wrap_use_style(bwrap::WrapStyle::MayBrk(None, None)).unwrap();
        println!("WRAPPED:\n\n{}", wrapped);


<a id="org869ba6e"></a>

## Append/prepend

Bwrap can append or prepend whatever string to newly added newline
character. With this feature, one can effectively achieve indentation,
line trailing notation or similar.


<a id="org3248f83"></a>

### Indentation

Original:

    Here is our schedule:
    - Do A, and do B, and do C, and do D, and do E, and do F
    - Do G, and do H, and do I, and do J, and do K, and do L

**Wrapped**:

    Here is our schedule:
    - Do A, and do B, and do C, and do
      D, and do E, and do F
    - Do G, and do H, and do I, and do
      J, and do K, and do L

Source code:

    use bwrap::{EasyWrapper, ExistNlPref, WrapStyle::NoBrk};
    
    let line = "Here is our schedule:\n- Do A, and do B, and do C, and do D, and do E, and do F\n- Do G, and do H, and do I, and do J, and do K, and do L";
    println!("ORIGINAL:\n\n{}\n", line);
    let mut w = EasyWrapper::new(line, 35).unwrap();
    let wrapped = w.wrap_use_style(NoBrk(Some("  "), ExistNlPref::KeepTrailSpc)).unwrap();
    println!("WRAPPED:\n\n{}", wrapped);


<a id="orgad88a98"></a>

### Trailing notation

Original:

    VGhpcyBpcyBhIHNlY3JldCBtZXNzYWdlLCBwbGVhc2UgZGVsZXRlIGFmdGVyIHJlYWQK

**Wrapped**:

    VGhpcyBpcy |
    BhIHNlY3Jl |
    dCBtZXNzYW |
    dlLCBwbGVh |
    c2UgZGVsZX |
    RlIGFmdGVy |
    IHJlYWQK  

Source code:

    use bwrap::{EasyWrapper, WrapStyle::MayBrk};
    
    let line = "VGhpcyBpcyBhIHNlY3JldCBtZXNzYWdlLCBwbGVhc2UgZGVsZXRlIGFmdGVyIHJlYWQK";
    println!("ORIGINAL:\n\n{}\n", line);
    let mut w = EasyWrapper::new(line, 10).unwrap();
    let wrapped = w.wrap_use_style(MayBrk(Some(" |"), None)).unwrap();
    println!("WRAPPED:\n\n{}", wrapped);


<a id="org2ca36a1"></a>

# License

Bwrap can be licensed under either [MIT License](https://github.com/imichael2e2/bwrap/blob/master/LICENSE-MIT) or [GNU General
Public License Version 3.0](https://github.com/imichael2e2/bwrap/blob/master/LICENSE-GPL). The choice is up to the recipients.

