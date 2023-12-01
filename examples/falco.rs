extern crate phalanx;

use std::env;
use std::process;

const TAIL_CALLED: [&str; 246] = [
    "sys_enter",
    "sys_exit",
    "accept_e",
    "accept4_e",
    "access_e",
    "bind_e",
    "bpf_e",
    "capset_e",
    "chdir_e",
    "chmod_e",
    "chown_e",
    "chroot_e",
    "clone_e",
    "clone3_e",
    "close_e",
    "connect_e",
    "copy_file_range_e",
    "creat_e",
    "dup_e",
    "dup2_e",
    "dup3_e",
    "epoll_create_e",
    "epoll_create1_e",
    "eventfd_e",
    "execve_e",
    "execveat_e",
    "fchdir_e",
    "fchmod_e",
    "fchmodat_e",
    "fchown_e",
    "fchownat_e",
    "fcntl_e",
    "flock_e",
    "fork_e",
    "fsconfig_e",
    "futex_e",
    "generic_e",
    "getegid_e",
    "geteuid_e",
    "getgid_e",
    "getresgid_e",
    "getresuid_e",
    "getsockopt_e",
    "getuid_e",
    "inotify_init_e",
    "io_uring_enter_e",
    "io_uring_register_e",
    "io_uring_setup_e",
    "ioctl_e",
    "kill_e",
    "lchown_e",
    "link_e",
    "linkat_e",
    "listen_e",
    "llseek_e",
    "lseek_e",
    "mkdir_e",
    "mkdirat_e",
    "mlock_e",
    "mlock2_e",
    "mlockall_e",
    "mmap_e",
    "mmap2_e",
    "mount_e",
    "mprotect_e",
    "munlock_e",
    "munlockall_e",
    "munmap_e",
    "nanosleep_e",
    "open_e",
    "open_by_handle_at_e",
    "openat_e",
    "openat2_e",
    "pipe_e",
    "poll_e",
    "ppoll_e",
    "prlimit64_e",
    "ptrace_e",
    "quotactl_e",
    "read_e",
    "recvfrom_e",
    "recvmmsg_e",
    "recvmsg_e",
    "rename_e",
    "renameat_e",
    "renameat2_e",
    "rmdir_e",
    "seccomp_e",
    "select_e",
    "semctl_e",
    "semget_e",
    "semop_e",
    "sendfile_e",
    "sendmmsg_e",
    "sendmsg_e",
    "sendto_e",
    "setgid_e",
    "setns_e",
    "setpgid_e",
    "setresgid_e",
    "setresuid_e",
    "setrlimit_e",
    "setsid_e",
    "setsockopt_e",
    "setuid_e",
    "shutdown_e",
    "signalfd_e",
    "socket_e",
    "socketpair_e",
    "splice_e",
    "stat_e",
    "symlink_e",
    "symlinkat_e",
    "tgkill_e",
    "timerfd_create_e",
    "tkill_e",
    "umount_e",
    "umount2_e",
    "unlink_e",
    "unlinkat_e",
    "unshare_e",
    "userfaultfd_e",
    "vfork_e",
    "write_e",
    "accept_x",
    "accept4_x",
    "access_x",
    "bind_x",
    "bpf_x",
    "capset_x",
    "chdir_x",
    "chmod_x",
    "chown_x",
    "chroot_x",
    "clone_x",
    "clone3_x",
    "close_x",
    "connect_x",
    "copy_file_range_x",
    "creat_x",
    "dup_x",
    "dup2_x",
    "dup3_x",
    "epoll_create_x",
    "epoll_create1_x",
    "eventfd_x",
    "execve_x",
    "execveat_x",
    "fchdir_x",
    "fchmod_x",
    "fchmodat_x",
    "fchown_x",
    "fchownat_x",
    "fcntl_x",
    "flock_x",
    "fork_x",
    "fsconfig_x",
    "futex_x",
    "generic_x",
    "getegid_x",
    "geteuid_x",
    "getgid_x",
    "getresgid_x",
    "getresuid_x",
    "getsockopt_x",
    "getuid_x",
    "inotify_init_x",
    "io_uring_xnter_x",
    "io_uring_register_x",
    "io_uring_setup_x",
    "ioctl_x",
    "kill_x",
    "lchown_x",
    "link_x",
    "linkat_x",
    "listen_x",
    "llseek_x",
    "lseek_x",
    "mkdir_x",
    "mkdirat_x",
    "mlock_x",
    "mlock2_x",
    "mlockall_x",
    "mmap_x",
    "mmap2_x",
    "mount_x",
    "mprotect_x",
    "munlock_x",
    "munlockall_x",
    "munmap_x",
    "nanosleep_x",
    "open_x",
    "open_by_handle_at_x",
    "openat_x",
    "openat2_x",
    "pipe_x",
    "poll_x",
    "ppoll_x",
    "prlimit64_x",
    "ptrace_x",
    "quotactl_x",
    "read_x",
    "recvfrom_x",
    "recvmmsg_x",
    "recvmsg_x",
    "rename_x",
    "renameat_x",
    "renameat2_x",
    "rmdir_x",
    "seccomp_x",
    "select_x",
    "semctl_x",
    "semget_x",
    "semop_x",
    "sendfile_x",
    "sendmmsg_x",
    "sendmsg_x",
    "sendto_x",
    "setgid_x",
    "setns_x",
    "setpgid_x",
    "setresgid_x",
    "setresuid_x",
    "setrlimit_x",
    "setsid_x",
    "setsockopt_x",
    "setuid_x",
    "shutdown_x",
    "signalfd_x",
    "socket_x",
    "socketpair_x",
    "splice_x",
    "stat_x",
    "symlink_x",
    "symlinkat_x",
    "tgkill_x",
    "timerfd_create_x",
    "tkill_x",
    "umount_x",
    "umount2_x",
    "unlink_x",
    "unlinkat_x",
    "unshare_x",
    "userfaultfd_x",
    "vfork_x",
    "write_x",
];

fn usage() -> ! {
    println!("{} <bpf-probe.o>", env::args().next().unwrap());
    process::exit(1);
}

fn main() {
    let path = env::args().skip(1).next();

    let ph = match path {
        Some(path) => phalanx::load(path.into()),
        None => usage(),
    };
}
