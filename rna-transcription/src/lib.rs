#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    sequence: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    sequence: String,
}

impl Dna {
    /// 从字符串创建 DNA 序列
    ///
    /// # 参数
    /// * `dna` - 包含 DNA 核苷酸的字符串（有效字符：A, C, G, T）
    ///
    /// # 返回
    /// * `Ok(Dna)` - 如果所有字符都是有效的 DNA 核苷酸
    /// * `Err(usize)` - 如果包含无效字符，返回第一个无效字符的字节索引
    pub fn new(dna: &str) -> Result<Dna, usize> {
        // 有效的 DNA 核苷酸：A, C, G, T
        for (index, ch) in dna.char_indices() {
            if !matches!(ch, 'A' | 'C' | 'G' | 'T') {
                return Err(index);
            }
        }
        Ok(Dna {
            sequence: dna.to_string(),
        })
    }

    /// 将 DNA 序列转换为 RNA 序列
    ///
    /// 转换规则：
    /// - A -> U
    /// - C -> G
    /// - G -> C
    /// - T -> A
    pub fn into_rna(self) -> Rna {
        let rna_sequence: String = self
            .sequence
            .chars()
            .map(|ch| match ch {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                _ => ch, // 理论上不应该发生，因为 new 已经验证过
            })
            .collect();
        Rna {
            sequence: rna_sequence,
        }
    }
}

impl Rna {
    /// 从字符串创建 RNA 序列
    ///
    /// # 参数
    /// * `rna` - 包含 RNA 核苷酸的字符串（有效字符：A, C, G, U）
    ///
    /// # 返回
    /// * `Ok(Rna)` - 如果所有字符都是有效的 RNA 核苷酸
    /// * `Err(usize)` - 如果包含无效字符，返回第一个无效字符的字节索引
    pub fn new(rna: &str) -> Result<Rna, usize> {
        // 有效的 RNA 核苷酸：A, C, G, U
        for (index, ch) in rna.char_indices() {
            if !matches!(ch, 'A' | 'C' | 'G' | 'U') {
                return Err(index);
            }
        }
        Ok(Rna {
            sequence: rna.to_string(),
        })
    }
}
