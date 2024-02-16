// clang-format off
//#include <vmlinux.h>
#include <linux/types.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
// clang-format on

SEC("tp/syscalls/sys_enter_close")
int close_e(void *ctx) {
  char fmt[] = "hello, bpf";
  bpf_trace_printk(fmt, sizeof(fmt));
}
