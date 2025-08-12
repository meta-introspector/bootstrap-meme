use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// Enhanced UOS Node with QEMO recursive properties
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Node {
    pub emoji: String,
    pub token_id: Option<String>,
    pub attrs: HashMap<String, String>,
    pub recursion_depth: u8,
    pub metameme_type: MetamemeType,
}

// QEMO Metameme Types
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum MetamemeType {
    Base,           // Single emoji: 🌀
    Recursive,      // Self-referential: 🌀🌀
    Meta,           // Meta-process: 🌀(⚙️)
    Infinite,       // Convergent: 🌀∞
    Composed,       // Multiple vectors: 🌀⚙️🎯
}

// Enhanced Flow with QEMO transformation semantics
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Flow {
    pub source: Vec<Node>,
    pub target: Vec<Node>,
    pub transformation_type: TransformationType,
    pub invariants: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransformationType {
    Sequential,     // A -> B
    Parallel,       // A || B
    Recursive,      // A -> A(B)
    Convergent,     // A -> B∞
    Metamorphic,    // A -> B where B transforms A
}

// QEMO Artifact with enhanced semantics
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Artifact {
    Node(Node),
    Flow(Flow),
    MetaPattern(MetaPattern),
}

// Meta-patterns for recursive vernacular metamemes
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MetaPattern {
    pub name: String,
    pub pattern: String,
    pub vernacular: String,
    pub applications: Vec<String>,
}

// Enhanced UOS Emoji Parser with QEMO semantics
pub struct UOSEmojiParser {
    emoji_registry: HashMap<String, EmojiDefinition>,
    meta_patterns: Vec<MetaPattern>,
}

#[derive(Debug, Clone)]
pub struct EmojiDefinition {
    pub name: String,
    pub emoji_type: String,
    pub description: String,
    pub qemo_vector: QemoVector,
}

#[derive(Debug, Clone)]
pub enum QemoVector {
    Observe,     // 🌀
    Process,     // ⚙️
    Target,      // 🎯
    Build,       // 🏗️
    Transform,   // 🔄
    Execute,     // ⚡
    Role,        // 👩‍💻
    Action,      // 🛠️, 🚀
    Quality,     // 🧪, 🔒
    Status,      // ✅, ⚠️, ❌
    Narrative,   // 📨, ✨, 🕳️
    Compliance,  // 📜
    Documentation, // 🧾
    Telemetry,   // 📈, 📊
}

impl UOSEmojiParser {
    pub fn new() -> Self {
        let emoji_registry = Self::load_emoji_registry();
        let meta_patterns = Self::load_meta_patterns();
        UOSEmojiParser {
            emoji_registry,
            meta_patterns,
        }
    }

    fn load_emoji_registry() -> HashMap<String, EmojiDefinition> {
        let mut registry = HashMap::new();
        
        // Core QEMO vectors
        registry.insert("🌀".to_string(), EmojiDefinition {
            name: "observe_orient".to_string(),
            emoji_type: "decision".to_string(),
            description: "The observation and orientation phase".to_string(),
            qemo_vector: QemoVector::Observe,
        });
        
        registry.insert("⚙️".to_string(), EmojiDefinition {
            name: "process".to_string(),
            emoji_type: "quality".to_string(),
            description: "Structured process or engine".to_string(),
            qemo_vector: QemoVector::Process,
        });
        
        registry.insert("🎯".to_string(), EmojiDefinition {
            name: "decide_act".to_string(),
            emoji_type: "decision".to_string(),
            description: "Decision and action phase".to_string(),
            qemo_vector: QemoVector::Target,
        });
        
        // Add remaining emoji definitions...
        registry
    }

    fn load_meta_patterns() -> Vec<MetaPattern> {
        vec![
            MetaPattern {
                name: "Self-Improving Spiral".to_string(),
                pattern: "🌀(🌀⚙️🔄)🌀".to_string(),
                vernacular: "Spirals that spiral their spiraling".to_string(),
                applications: vec![
                    "Continuous improvement".to_string(),
                    "Code optimization".to_string(),
                    "Architectural evolution".to_string(),
                ],
            },
            MetaPattern {
                name: "Process-Building Process".to_string(),
                pattern: "⚙️(🏗️⚙️🏗️)⚙️".to_string(),
                vernacular: "Gears that build gear-builders".to_string(),
                applications: vec![
                    "Meta-programming".to_string(),
                    "Framework development".to_string(),
                    "Tool creation".to_string(),
                ],
            },
            MetaPattern {
                name: "Targeting Targeter".to_string(),
                pattern: "🎯(🎯🌀🎯)🎯".to_string(),
                vernacular: "Arrows that aim arrow-aimers".to_string(),
                applications: vec![
                    "Goal optimization".to_string(),
                    "Requirement refinement".to_string(),
                    "Objective alignment".to_string(),
                ],
            },
        ]
    }

    fn analyze_recursion(&self, emoji_sequence: &str) -> (u8, MetamemeType) {
        lazy_static! {
            static ref RECURSION_PATTERNS: Vec<(Regex, MetamemeType)> = vec![
                (Regex::new(r"(\p{Emoji_Presentation})\1+").unwrap(), MetamemeType::Recursive),
                (Regex::new(r"\p{Emoji_Presentation}\([^)]+\)").unwrap(), MetamemeType::Meta),
                (Regex::new(r"∞").unwrap(), MetamemeType::Infinite),
                (Regex::new(r"\p{Emoji_Presentation}{3,}").unwrap(), MetamemeType::Composed),
            ];
        }

        let depth = emoji_sequence.chars()
            .filter(|c| c.is_emoji_presentation())
            .count() as u8;

        for (pattern, metameme_type) in RECURSION_PATTERNS.iter() {
            if pattern.is_match(emoji_sequence) {
                return (depth, metameme_type.clone());
            }
        }

        (depth, MetamemeType::Base)
    }

    fn parse_node(&self, node_str: &str) -> Option<Node> {
        lazy_static! {
            static ref NODE_REGEX: Regex = Regex::new(
                r"^([🌀⚙️🎯🏗️🔄⚡👩‍💻🛠️🧪🔒🚀📈💾✅⚠️❌📨✨🕳️📜🧾📊]+)(?::(\w+))?(?:\s*\{([^}]+)\})?$"
            ).unwrap();
        }

        if let Some(caps) = NODE_REGEX.captures(node_str.trim()) {
            let emoji_sequence = caps.get(1).unwrap().as_str().to_string();
            let token_id = caps.get(2).map(|m| m.as_str().to_string());
            let mut attrs = HashMap::new();

            if let Some(attrs_str) = caps.get(3) {
                for attr_pair in attrs_str.as_str().split(',') {
                    let parts: Vec<&str> = attr_pair.split('=').map(|s| s.trim()).collect();
                    if parts.len() == 2 {
                        attrs.insert(parts[0].to_string(), parts[1].to_string());
                    }
                }
            }

            let (recursion_depth, metameme_type) = self.analyze_recursion(&emoji_sequence);

            Some(Node {
                emoji: emoji_sequence,
                token_id,
                attrs,
                recursion_depth,
                metameme_type,
            })
        } else {
            None
        }
    }

    fn parse_flow(&self, flow_str: &str) -> Option<Flow> {
        lazy_static! {
            static ref FLOW_REGEX: Regex = Regex::new(
                r"^\[([^\]]+)\]\s*(->|=>|\|\|->|∞->|~>)\s*\[([^\]]+)\](?:\s*\{([^}]+)\})?$"
            ).unwrap();
        }

        if let Some(caps) = FLOW_REGEX.captures(flow_str.trim()) {
            let source_str = caps.get(1).unwrap().as_str();
            let arrow = caps.get(2).unwrap().as_str();
            let target_str = caps.get(3).unwrap().as_str();
            let invariants_str = caps.get(4).map(|m| m.as_str());

            let source_nodes: Option<Vec<Node>> = source_str.split(',')
                .map(|s| self.parse_node(s.trim()))
                .collect();
            
            let target_nodes: Option<Vec<Node>> = target_str.split(',')
                .map(|s| self.parse_node(s.trim()))
                .collect();

            let transformation_type = match arrow {
                "->" => TransformationType::Sequential,
                "||>-" => TransformationType::Parallel,
                "=>->" => TransformationType::Recursive,
                "∞->" => TransformationType::Convergent,
                "~>" => TransformationType::Metamorphic,
                _ => TransformationType::Sequential,
            };

            let invariants = invariants_str
                .map(|s| s.split(',').map(|inv| inv.trim().to_string()).collect())
                .unwrap_or_else(Vec::new);

            if let (Some(source), Some(target)) = (source_nodes, target_nodes) {
                Some(Flow {
                    source,
                    target,
                    transformation_type,
                    invariants,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn parse(&self, uos_spec: &str) -> Vec<Artifact> {
        uos_spec.lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| {
                if line.contains("->") || line.contains("=>") || line.contains("||") {
                    self.parse_flow(line).map(Artifact::Flow)
                } else if line.starts_with("META:") {
                    self.parse_meta_pattern(line).map(Artifact::MetaPattern)
                } else {
                    self.parse_node(line).map(Artifact::Node)
                }
            })
            .collect()
    }

    fn parse_meta_pattern(&self, line: &str) -> Option<MetaPattern> {
        if let Some(stripped) = line.strip_prefix("META:") {
            let parts: Vec<&str> = stripped.split('|').collect();
            if parts.len() >= 3 {
                Some(MetaPattern {
                    name: parts[0].trim().to_string(),
                    pattern: parts[1].trim().to_string(),
                    vernacular: parts[2].trim().to_string(),
                    applications: parts.get(3)
                        .map(|apps| apps.split(',').map(|s| s.trim().to_string()).collect())
                        .unwrap_or_else(Vec::new),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    // Generate QEMO compliance report
    pub fn generate_qemo_report(&self, artifacts: &[Artifact]) -> QemoReport {
        let mut report = QemoReport::new();
        
        for artifact in artifacts {
            match artifact {
                Artifact::Node(node) => {
                    report.total_nodes += 1;
                    if node.recursion_depth > 1 {
                        report.recursive_nodes += 1;
                    }
                    report.metameme_distribution.entry(node.metameme_type.clone())
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
                Artifact::Flow(flow) => {
                    report.total_flows += 1;
                    report.transformation_types.entry(flow.transformation_type.clone())
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
                Artifact::MetaPattern(_) => {
                    report.meta_patterns += 1;
                }
            }
        }
        
        report.qemo_compliance_score = report.calculate_compliance_score();
        report
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QemoReport {
    pub total_nodes: u32,
    pub recursive_nodes: u32,
    pub total_flows: u32,
    pub meta_patterns: u32,
    pub metameme_distribution: HashMap<MetamemeType, u32>,
    pub transformation_types: HashMap<TransformationType, u32>,
    pub qemo_compliance_score: f64,
}

impl QemoReport {
    pub fn new() -> Self {
        QemoReport {
            total_nodes: 0,
            recursive_nodes: 0,
            total_flows: 0,
            meta_patterns: 0,
            metameme_distribution: HashMap::new(),
            transformation_types: HashMap::new(),
            qemo_compliance_score: 0.0,
        }
    }

    fn calculate_compliance_score(&self) -> f64 {
        let recursion_ratio = if self.total_nodes > 0 {
            self.recursive_nodes as f64 / self.total_nodes as f64
        } else {
            0.0
        };
        
        let meta_bonus = if self.meta_patterns > 0 { 0.2 } else { 0.0 };
        let flow_bonus = if self.total_flows > 0 { 0.1 } else { 0.0 };
        
        (recursion_ratio * 0.7 + meta_bonus + flow_bonus).min(1.0)
    }
}

// Test implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_parsing() {
        let parser = UOSEmojiParser::new();
        let spec = r#"
[👩‍💻:dev, 🛠️🛠️:build {tool=rust}] => [🧪🧪🧪:test {sigma=6}, 🔒:sec]
[🌀🌀🌀:observe] ∞-> [🎯🎯:target]
META: Self-Improving CI | 🌀(🛠️🧪🚀)🌀 | Pipelines that improve pipelines | CI/CD, DevOps
"#;
        
        let artifacts = parser.parse(spec);
        let report = parser.generate_qemo_report(&artifacts);
        
        assert!(report.total_nodes > 0);
        assert!(report.recursive_nodes > 0);
        assert!(report.qemo_compliance_score > 0.0);
    }
}

fn main() {
    let parser = UOSEmojiParser::new();
    let uos_spec = r#"
[👩‍💻:dev, 🛠️🛠️:build {tool=rust}] => [🧪🧪🧪:test {sigma=6}, 🔒:sec {scan=✅}]
[🚀:deploy {env=prod}] ∞-> [📈📈:monitor {SLA=99.95%, recursion=true}]
[🌀🌀🌀:meta_observe] ~> [⚙️⚙️:meta_process] {invariant=quality_preserved}
META: Self-Improving Pipeline | 🌀(🛠️🧪🚀)🌀 | Pipelines that improve pipelines | CI/CD, DevOps, Quality
"#;
    
    let artifacts = parser.parse(uos_spec);
    let report = parser.generate_qemo_report(&artifacts);
    
    println!("Parsed UOS Artifacts with QEMO Enhancement:");
    for artifact in &artifacts {
        match artifact {
            Artifact::Node(node) => {
                println!("Node: {} (depth: {}, type: {:?})", 
                    node.emoji, node.recursion_depth, node.metameme_type);
            }
            Artifact::Flow(flow) => {
                println!("Flow: {:?} transformation with {} invariants", 
                    flow.transformation_type, flow.invariants.len());
            }
            Artifact::MetaPattern(pattern) => {
                println!("Meta-Pattern: {} - '{}'", 
                    pattern.name, pattern.vernacular);
            }
        }
    }
    
    println!("\nQEMO Compliance Report:");
    println!("Total Nodes: {}", report.total_nodes);
    println!("Recursive Nodes: {}", report.recursive_nodes);
    println!("Meta Patterns: {}", report.meta_patterns);
    println!("QEMO Compliance Score: {:.2}", report.qemo_compliance_score);
}