// clang-format off
#include <vmlinux.h>
#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
// clang-format on

SEC("tp/syscalls/sys_enter_close")
BPF_PROG(close_e) {
  char[] fmt = "hello, bpf";
  bpf_trace_printk(fmt, sizeof(fmt));
}
