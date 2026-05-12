# ZK 压缩装备与权限管理系统

这是一个基于 **Anchor** 框架和 **Light Protocol (ZK 压缩)** 技术构建的高性能、模块化 Solana 程序。该系统为管理权限、功能配置以及可扩展的资产铸造（装备盒）提供了一套完整的工业级解决方案。

## 🏗 系统架构

程序采用分层设计，确保安全性、灵活性和成本效益：

1.  **权限层 (`AuthorityRegistry`)**:
    *   基于 `feature_id` 的去中心化权限管理。
    *   支持**两步转移机制**（提议与接受），防止因地址填错导致的权限永久丢失。
2.  **元数据层 (`IpfsConfig`)**:
    *   将链下 IPFS URI 与特定功能绑定。
    *   支持 `realloc` 动态调整存储空间。
3.  **运营层 (`EquipmentConfig`)**:
    *   提供业务逻辑的全局开关（如“暂停服务”和“开启铸造”）。
    *   作为紧急情况下的“断路器”。
4.  **资产层 (`ArtifactBox`)**:
    *   利用 **ZK 压缩技术**将装备 random_seed 数据存储在压缩状态树中。
    *   相比传统的 Solana 账户，大幅降低了存储开销（Rent）。

---

## 🚀 核心特性

-   **模块化治理**: 通过唯一的 `feature_id` 字符串隔离不同游戏模块或服务的权限。
-   **安全权限转移**: 管理员更替需要新管理员签署“接受”交易，确保目标地址有效且活跃。
-   **ZK 压缩资产**: 结合 Light Protocol 铸造“装备盒”，以极低的成本支持数百万量级的资产。
-   **运营控制**: 管理员可以通过 `EquipmentConfig` 立即暂停特定功能或禁用铸造。

---

## 🛠 指令参考

程序指令分为以下三个功能模块：

### 1. 权限注册表 (治理模块)
通过安全的“两步转移”流程管理特定功能的管理权限。

| 指令名称 | 参数 | 签名者 | 功能描述 |
| :--- | :--- | :--- | :--- |
| `authority_registry_initialize` | `feature_id` | 初始管理员 | 为特定功能初始化权限。 |
| `authority_registry_propose` | `feature_id` | 现任管理员 | 提议新的管理员地址。 |
| `authority_registry_accept` | `feature_id` | 新管理员 | 新管理员接受并完成权限交接。 |
| `authority_registry_cancel` | `feature_id` | 管理员/新候选人 | 取消进行中的权限转移提议。 |

### 2. 配置与运营控制
管理元数据链接和业务逻辑开关（断路器）。

| 指令名称 | 参数 | 签名者 | 功能描述 |
| :--- | :--- | :--- | :--- |
| `ipfs_config_initialize` | `feature_id`, `config_uri` | 功能管理员 | 设置装备的 IPFS 元数据链接。 |
| `ipfs_config_update` | `feature_id`, `config_uri` | 功能管理员 | 更新 IPFS 链接（支持账户扩容）。 |
| `equipment_config_initialize` | `feature_id` | 功能管理员 | 初始化业务开关（暂停/铸造）。 |
| `equipment_config_update` | `feature_id`, `pause?`, `mint_enabled?` | 功能管理员 | 更新业务运营状态。 |

### 3. ZK 资产铸造 (Light Protocol)
利用零知识证明技术铸造压缩资产。

| 指令名称 | 参数 | 功能描述 |
| :--- | :--- | :--- |
| `generate_artifact_box` | `proof_bytes`, `address_tree_info_bytes`, `output_state_tree_index`, `artifact_id`, `commitment` | 铸造一个 ZK 压缩装备盒。程序内部会将字节流反序列化为有效性证明和地址树信息并与 Light Protocol 交互。 |

---

## 🔑 关键参数说明
在调用 `generate_artifact_box` 时，`proof_bytes` 和 `address_tree_info_bytes` 必须先通过 Borsh 进行序列化。程序内部会校验这些证明文件，确保其符合 Light Protocol 状态树的约束。

---

## 💻 开发指南

- [Anchor CLI](https://www.anchor-lang.com/)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Light Protocol SDK](https://www.lightprotocol.com/)

---
<div align="center">
  <sub>此文档由 AI (Gemini 3.1) 辅助生成。</sub></br>
</div>