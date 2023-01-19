use lazy_static::lazy_static;
lazy_static! {
    pub static ref EMOTES: HashMap<&'static str, &'static str> = [
        ("无", ""),
        ("代码框", "```\n```"),
        ("|∀ﾟ", "|∀ﾟ"),
        ("(´ﾟДﾟ`)", "(´ﾟДﾟ`)"),
        ("(;´Д`)", "(;´Д`)"),
        ("(｀･ω･)", "(｀･ω･)"),
        ("(=ﾟωﾟ)=", "(=ﾟωﾟ)="),
        ("| ω・´)", "| ω・´)"),
        ("|-` )", "|-` )"),
        ("|д` )", "|д` )"),
        ("|ー` )", "|ー` )"),
        ("|∀` )", "|∀` )"),
        ("(つд⊂)", "(つд⊂)"),
        ("(ﾟДﾟ≡ﾟДﾟ)", "(ﾟДﾟ≡ﾟДﾟ)"),
        ("(＾o＾)ﾉ", "(＾o＾)ﾉ"),
        ("(|||ﾟДﾟ)", "(|||ﾟДﾟ)"),
        ("( ﾟ∀ﾟ)", "( ﾟ∀ﾟ)"),
        ("( ´∀`)", "( ´∀`)"),
        ("(*´∀`)", "(*´∀`)"),
        ("(*ﾟ∇ﾟ)", "(*ﾟ∇ﾟ)"),
        ("(*ﾟーﾟ)", "(*ﾟーﾟ)"),
        ("(　ﾟ 3ﾟ)", "(　ﾟ 3ﾟ)"),
        ("( ´ー`)", "( ´ー`)"),
        ("( ・_ゝ・)", "( ・_ゝ・)"),
        ("( ´_ゝ`)", "( ´_ゝ`)"),
        ("(*´д`)", "(*´д`)"),
        ("(・ー・)", "(・ー・)"),
        ("(・∀・)", "(・∀・)"),
        ("(ゝ∀･)", "(ゝ∀･)"),
        ("(〃∀〃)", "(〃∀〃)"),
        ("(*ﾟ∀ﾟ*)", "(*ﾟ∀ﾟ*)"),
        ("( ﾟ∀。)", "( ﾟ∀。)"),
        ("( `д´)", "( `д´)"),
        ("(`ε´ )", "(`ε´ )"),
        ("(`ヮ´ )", "(`ヮ´ )"),
        ("σ`∀´)", "σ`∀´)"),
        (" ﾟ∀ﾟ)σ", " ﾟ∀ﾟ)σ"),
        ("ﾟ ∀ﾟ)ノ", "ﾟ ∀ﾟ)ノ"),
        ("(╬ﾟдﾟ)", "(╬ﾟдﾟ)"),
        ("(|||ﾟдﾟ)", "(|||ﾟдﾟ)"),
        ("( ﾟдﾟ)", "( ﾟдﾟ)"),
        ("Σ( ﾟдﾟ)", "Σ( ﾟдﾟ)"),
        ("( ;ﾟдﾟ)", "( ;ﾟдﾟ)"),
        ("( ;´д`)", "( ;´д`)"),
        ("(　д ) ﾟ ﾟ", "(　д ) ﾟ ﾟ"),
        ("( ☉д⊙)", "( ☉д⊙)"),
        ("(((　ﾟдﾟ)))", "(((　ﾟдﾟ)))"),
        ("( ` ・´)", "( ` ・´)"),
        ("( ´д`)", "( ´д`)"),
        ("( -д-)", "( -д-)"),
        ("(>д<)", "(>д<)"),
        ("･ﾟ( ﾉд`ﾟ)", "･ﾟ( ﾉд`ﾟ)"),
        ("( TдT)", "( TдT)"),
        ("(￣∇￣)", "(￣∇￣)"),
        ("(￣3￣)", "(￣3￣)"),
        ("(￣ｰ￣)", "(￣ｰ￣)"),
        ("(￣ . ￣)", "(￣ . ￣)"),
        ("(￣皿￣)", "(￣皿￣)"),
        ("(￣艸￣)", "(￣艸￣)"),
        ("(￣︿￣)", "(￣︿￣)"),
        ("(￣︶￣)", "(￣︶￣)"),
        ("ヾ(´ωﾟ｀)", "ヾ(´ωﾟ｀)"),
        ("(*´ω`*)", "(*´ω`*)"),
        ("(・ω・)", "(・ω・)"),
        ("( ´・ω)", "( ´・ω)"),
        ("(｀・ω)", "(｀・ω)"),
        ("(´・ω・`)", "(´・ω・`)"),
        ("(`・ω・´)", "(`・ω・´)"),
        ("( `_っ´)", "( `_っ´)"),
        ("( `ー´)", "( `ー´)"),
        ("( ´_っ`)", "( ´_っ`)"),
        ("( ´ρ`)", "( ´ρ`)"),
        ("( ﾟωﾟ)", "( ﾟωﾟ)"),
        ("(oﾟωﾟo)", "(oﾟωﾟo)"),
        ("(　^ω^)", "(　^ω^)"),
        ("(｡◕∀◕｡)", "(｡◕∀◕｡)"),
        ("/( ◕‿‿◕ )\\", "/( ◕‿‿◕ )\\"),
        ("ヾ(´ε`ヾ)", "ヾ(´ε`ヾ)"),
        ("(ノﾟ∀ﾟ)ノ", "(ノﾟ∀ﾟ)ノ"),
        ("(σﾟдﾟ)σ", "(σﾟдﾟ)σ"),
        ("(σﾟ∀ﾟ)σ", "(σﾟ∀ﾟ)σ"),
        ("|дﾟ )", "|дﾟ )"),
        ("┃電柱┃", "┃電柱┃"),
        ("ﾟ(つд`ﾟ)", "ﾟ(つд`ﾟ)"),
        ("ﾟÅﾟ )　", "ﾟÅﾟ )　"),
        ("⊂彡☆))д`)", "⊂彡☆))д`)"),
        ("⊂彡☆))д´)", "⊂彡☆))д´)"),
        ("⊂彡☆))∀`)", "⊂彡☆))∀`)"),
        ("(´∀((☆ミつ", "(´∀((☆ミつ"),
        ("･ﾟ( ﾉヮ´ )", "･ﾟ( ﾉヮ´ )"),
        ("(ﾉ)`ω´(ヾ)", "(ﾉ)`ω´(ヾ)"),
        ("ᕕ( ᐛ )ᕗ", "ᕕ( ᐛ )ᕗ"),
        ("(　ˇωˇ)", "(　ˇωˇ)"),
        ("( ｣ﾟДﾟ)｣＜", "( ｣ﾟДﾟ)｣＜"),
        ("( ›´ω`‹ )", "( ›´ω`‹ )"),
        ("(;´ヮ`)7", "(;´ヮ`)7"),
        ("(`ゥ´ )", "(`ゥ´ )"),
        ("(`ᝫ´ )", "(`ᝫ´ )"),
        ("( ᑭ`д´)ᓀ))д´)ᑫ", "( ᑭ`д´)ᓀ))д´)ᑫ"),
        ("σ( ᑒ )", "σ( ᑒ )"),
        ("齐齐蛤尔", "(`ヮ´ )σ`∀´) ﾟ∀ﾟ)σ"),
        (
            "大嘘",
            "吁~~~~　　rnm，退钱！\n 　　　/　　　/\n(　ﾟ 3ﾟ) `ー´) `д´) `д´)"
        ),
        ("防剧透", "[h] [/h]"),
        ("骰子", "[n]"),
        ("高级骰子", "[n,m]"),
    ]
    .iter()
    .copied()
    .collect();
}
use std::collections::HashMap;

/*

// update on 2023-01-19, copyright (?) nmbxd.com

<option value="|∀ﾟ">|∀ﾟ</option>
<option value="(´ﾟДﾟ`)">(´ﾟДﾟ`)</option>
<option value="(;´Д`)">(;´Д`)</option>
<option value="(｀･ω･)">(｀･ω･)</option>
<option value="(=ﾟωﾟ)=">(=ﾟωﾟ)=</option>
<option value="| ω・´)">| ω・´)</option>
<option value="|-` )">|-` )</option>
<option value="|д` )">|д` )</option>
<option value="|ー` )">|ー` )</option>
<option value="|∀` )">|∀` )</option>
<option value="(つд⊂)">(つд⊂)</option>
<option value="(ﾟДﾟ≡ﾟДﾟ)">(ﾟДﾟ≡ﾟДﾟ)</option>
<option value="(＾o＾)ﾉ">(＾o＾)ﾉ</option>
<option value="(|||ﾟДﾟ)">(|||ﾟДﾟ)</option>
<option value="( ﾟ∀ﾟ)">( ﾟ∀ﾟ)</option>
<option value="( ´∀`)">( ´∀`)</option>
<option value="(*´∀`)">(*´∀`)</option>
<option value="(*ﾟ∇ﾟ)">(*ﾟ∇ﾟ)</option>
<option value="(*ﾟーﾟ)">(*ﾟーﾟ)</option>
<option value="(　ﾟ 3ﾟ)">(　ﾟ 3ﾟ)</option>
<option value="( ´ー`)">( ´ー`)</option>
<option value="( ・_ゝ・)">( ・_ゝ・)</option>
<option value="( ´_ゝ`)">( ´_ゝ`)</option>
<option value="(*´д`)">(*´д`)</option>
<option value="(・ー・)">(・ー・)</option>
<option value="(・∀・)">(・∀・)</option>
<option value="(ゝ∀･)">(ゝ∀･)</option>
<option value="(〃∀〃)">(〃∀〃)</option>
<option value="(*ﾟ∀ﾟ*)">(*ﾟ∀ﾟ*)</option>
<option value="( ﾟ∀。)">( ﾟ∀。)</option>
<option value="( `д´)">( `д´)</option>
<option value="(`ε´ )">(`ε´ )</option>
<option value="(`ヮ´ )">(`ヮ´ )</option>
<option value="σ`∀´)">σ`∀´)</option>
<option value=" ﾟ∀ﾟ)σ"> ﾟ∀ﾟ)σ</option>
<option value="ﾟ ∀ﾟ)ノ">ﾟ ∀ﾟ)ノ</option>
<option value="(╬ﾟдﾟ)">(╬ﾟдﾟ)</option>
<option value="(|||ﾟдﾟ)">(|||ﾟдﾟ)</option>
<option value="( ﾟдﾟ)">( ﾟдﾟ)</option>
<option value="Σ( ﾟдﾟ)">Σ( ﾟдﾟ)</option>
<option value="( ;ﾟдﾟ)">( ;ﾟдﾟ)</option>
<option value="( ;´д`)">( ;´д`)</option>
<option value="(　д ) ﾟ ﾟ">(　д ) ﾟ ﾟ</option>
<option value="( ☉д⊙)">( ☉д⊙)</option>
<option value="(((　ﾟдﾟ)))">(((　ﾟдﾟ)))</option>
<option value="( ` ・´)">( ` ・´)</option>
<option value="( ´д`)">( ´д`)</option>
<option value="( -д-)">( -д-)</option>
<option value="(>д<)">(&gt;д&lt;)</option>
<option value="･ﾟ( ﾉд`ﾟ)">･ﾟ( ﾉд`ﾟ)</option>
<option value="( TдT)">( TдT)</option>
<option value="(￣∇￣)">(￣∇￣)</option>
<option value="(￣3￣)">(￣3￣)</option>
<option value="(￣ｰ￣)">(￣ｰ￣)</option>
<option value="(￣ . ￣)">(￣ . ￣)</option>
<option value="(￣皿￣)">(￣皿￣)</option>
<option value="(￣艸￣)">(￣艸￣)</option>
<option value="(￣︿￣)">(￣︿￣)</option>
<option value="(￣︶￣)">(￣︶￣)</option>
<option value="ヾ(´ωﾟ｀)">ヾ(´ωﾟ｀)</option>
<option value="(*´ω`*)">(*´ω`*)</option>
<option value="(・ω・)">(・ω・)</option>
<option value="( ´・ω)">( ´・ω)</option>
<option value="(｀・ω)">(｀・ω)</option>
<option value="(´・ω・`)">(´・ω・`)</option>
<option value="(`・ω・´)">(`・ω・´)</option>
<option value="( `_っ´)">( `_っ´)</option>
<option value="( `ー´)">( `ー´)</option>
<option value="( ´_っ`)">( ´_っ`)</option>
<option value="( ´ρ`)">( ´ρ`)</option>
<option value="( ﾟωﾟ)">( ﾟωﾟ)</option>
<option value="(oﾟωﾟo)">(oﾟωﾟo)</option>
<option value="(　^ω^)">(　^ω^)</option>
<option value="(｡◕∀◕｡)">(｡◕∀◕｡)</option>
<option value="/( ◕‿‿◕ )\">/( ◕‿‿◕ )\</option>
<option value="ヾ(´ε`ヾ)">ヾ(´ε`ヾ)</option>
<option value="(ノﾟ∀ﾟ)ノ">(ノﾟ∀ﾟ)ノ</option>
<option value="(σﾟдﾟ)σ">(σﾟдﾟ)σ</option>
<option value="(σﾟ∀ﾟ)σ">(σﾟ∀ﾟ)σ</option>
<option value="|дﾟ )">|дﾟ )</option>
<option value="┃電柱┃">┃電柱┃</option>
<option value="ﾟ(つд`ﾟ)">ﾟ(つд`ﾟ)</option>
<option value="ﾟÅﾟ )　">ﾟÅﾟ )　</option>
<option value="⊂彡☆))д`)">⊂彡☆))д`)</option>
<option value="⊂彡☆))д´)">⊂彡☆))д´)</option>
<option value="⊂彡☆))∀`)">⊂彡☆))∀`)</option>
<option value="(´∀((☆ミつ">(´∀((☆ミつ</option>
<option value="･ﾟ( ﾉヮ´ )">･ﾟ( ﾉヮ´ )</option>
<option value="(ﾉ)`ω´(ヾ)">(ﾉ)`ω´(ヾ)</option>
<option value="ᕕ( ᐛ )ᕗ">ᕕ( ᐛ )ᕗ</option>
<option value="(　ˇωˇ)">(　ˇωˇ)</option>
<option value="( ｣ﾟДﾟ)｣＜">( ｣ﾟДﾟ)｣＜</option>
<option value="( ›´ω`‹ )">( ›´ω`‹ )</option>
<option value="(;´ヮ`)7">(;´ヮ`)7</option>
<option value="(`ゥ´ )">(`ゥ´ )</option>
<option value="(`ᝫ´ )">(`ᝫ´ )</option>
<option value="( ᑭ`д´)ᓀ))д´)ᑫ">( ᑭ`д´)ᓀ))д´)ᑫ</option>
<option value="σ( ᑒ )">σ( ᑒ )</option>
<option value="(`ヮ´ )σ`∀´) ﾟ∀ﾟ)σ">齐齐蛤尔</option>
<option value="吁~~~~　　rnm，退钱！
/　　　/
(　ﾟ 3ﾟ) `ー´) `д´) `д´)">大嘘</option>
<option value="[h] [/h]">防剧透</option>
<option value="[n]">骰子</option>
<option value="[n,m]">高级骰子</option>

*/
