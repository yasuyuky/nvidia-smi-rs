mod structs;

pub use structs::NvidiaSmiMemResult;

#[cfg(test)]
mod tests {
    use structs::NvidiaSmiMemResult;

    #[test]
    fn it_parse() {
        let nvidia_smi_q_d_memory = r#"
==============NVSMI LOG==============

Timestamp                           : Sat May  5 21:36:53 2018
Driver Version                      : 384.81

Attached GPUs                       : 4
GPU 00000000:02:00.0
    FB Memory Usage
        Total                       : 12189 MiB
        Used                        : 0 MiB
        Free                        : 12189 MiB
    BAR1 Memory Usage
        Total                       : 256 MiB
        Used                        : 2 MiB
        Free                        : 254 MiB

GPU 00000000:03:00.0
    FB Memory Usage
        Total                       : 12189 MiB
        Used                        : 0 MiB
        Free                        : 12189 MiB
    BAR1 Memory Usage
        Total                       : 256 MiB
        Used                        : 2 MiB
        Free                        : 254 MiB

GPU 00000000:83:00.0
    FB Memory Usage
        Total                       : 12189 MiB
        Used                        : 0 MiB
        Free                        : 12189 MiB
    BAR1 Memory Usage
        Total                       : 256 MiB
        Used                        : 2 MiB
        Free                        : 254 MiB

GPU 00000000:84:00.0
    FB Memory Usage
        Total                       : 12189 MiB
        Used                        : 0 MiB
        Free                        : 12189 MiB
    BAR1 Memory Usage
        Total                       : 256 MiB
        Used                        : 2 MiB
        Free                        : 254 MiB

        "#;
        let nvidia_smi_mem_result = nvidia_smi_q_d_memory.parse::<NvidiaSmiMemResult>();
        assert!(nvidia_smi_mem_result.is_ok())
    }
}
