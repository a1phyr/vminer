/* Generated with cbindgen:0.24.3 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum LogLevel {
  LogLevelError,
  LogLevelWarn,
  LogLevelInfo,
  LogLevelDebug,
  LogLevelTrace,
} LogLevel;

typedef struct Backend Backend;

typedef struct Error Error;

typedef struct Os Os;

typedef struct Symbols Symbols;

typedef struct VmaFlags VmaFlags;

typedef struct PhysicalAddress {
  uint64_t val;
} PhysicalAddress;

typedef struct MemoryMap {
  struct PhysicalAddress start;
  struct PhysicalAddress end;
} MemoryMap;

typedef struct MemoryMapping {
  const struct MemoryMap *maps;
  uintptr_t len;
} MemoryMapping;

typedef struct X86_64Registers {
  uint64_t rax;
  uint64_t rbx;
  uint64_t rcx;
  uint64_t rdx;
  uint64_t rsi;
  uint64_t rdi;
  uint64_t rsp;
  uint64_t rbp;
  uint64_t r8;
  uint64_t r9;
  uint64_t r10;
  uint64_t r11;
  uint64_t r12;
  uint64_t r13;
  uint64_t r14;
  uint64_t r15;
  uint64_t rip;
  uint64_t rflags;
} X86_64Registers;

typedef struct X86_64Segment {
  uint64_t base;
  uint32_t limit;
  uint16_t selector;
  uint8_t type_;
  uint8_t present;
  uint8_t dpl;
  uint8_t db;
  uint8_t s;
  uint8_t l;
  uint8_t g;
  uint8_t avl;
  uint8_t unusable;
  uint8_t padding;
} X86_64Segment;

typedef struct X86_64Dtable {
  uint64_t base;
  uint16_t limit;
  uint16_t padding[3];
} X86_64Dtable;

typedef struct X86_64SpecialRegisters {
  struct X86_64Segment cs;
  struct X86_64Segment ds;
  struct X86_64Segment es;
  struct X86_64Segment fs;
  struct X86_64Segment gs;
  struct X86_64Segment ss;
  struct X86_64Segment tr;
  struct X86_64Segment ldt;
  struct X86_64Dtable gdt;
  struct X86_64Dtable idt;
  uint64_t cr0;
  uint64_t cr2;
  uint64_t cr3;
  uint64_t cr4;
  uint64_t cr8;
  uint64_t efer;
  uint64_t apic_base;
  uint64_t interrupt_bitmap[4];
} X86_64SpecialRegisters;

typedef struct X86_64OtherRegisters {
  uint64_t lstar;
  uint64_t gs_kernel_base;
} X86_64OtherRegisters;

typedef struct X86_64Vcpu {
  struct X86_64Registers registers;
  struct X86_64SpecialRegisters special_registers;
  struct X86_64OtherRegisters other_registers;
} X86_64Vcpu;

typedef struct X86_64Vcpus {
  const struct X86_64Vcpu *pointer;
  uintptr_t len;
} X86_64Vcpus;

typedef struct X86_64Backend {
  void *data;
  int32_t (*read_memory)(const void *data, struct PhysicalAddress addr, void *buf, uintptr_t size);
  struct MemoryMapping (*memory_mapping)(const void *data);
  struct X86_64Vcpus (*get_vcpus)(const void *data);
  void (*drop)(void *data);
} X86_64Backend;

#if defined(CUSTOM_ALLOCATOR)
typedef struct Allocator {
  void *data;
  void *(*alloc)(void*, uintptr_t, uintptr_t);
  void (*dealloc)(void*, void*, uintptr_t, uintptr_t);
  void *(*realloc)(void*, void*, uintptr_t, uintptr_t, uintptr_t);
} Allocator;
#endif

typedef struct LogRecord {
  enum LogLevel level;
  const char *message;
  const char *target;
  const char *file;
  uint32_t line;
} LogRecord;

typedef struct Logger {
  void *data;
  int (*enabled)(void *data, enum LogLevel level);
  void (*log)(void *data, const struct LogRecord *message);
  void (*flush)(void *data);
} Logger;

typedef struct VirtualAddress {
  uint64_t val;
} VirtualAddress;

typedef struct Process {
  struct VirtualAddress addr;
} Process;

typedef struct Thread {
  struct VirtualAddress addr;
} Thread;

typedef struct Vma {
  struct VirtualAddress addr;
} Vma;

typedef struct Module {
  struct VirtualAddress addr;
} Module;

typedef struct StackFrame {
  struct VirtualAddress ip;
  struct VirtualAddress sp;
} StackFrame;







#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct Backend *backend_make(struct X86_64Backend backend);

#if defined(STD)
struct Error *kvm_connect(int32_t pid, struct Backend **kvm);
#endif

#if defined(STD)
struct Error *read_dump(const char *path, struct Backend **dump);
#endif

void backend_free(struct Backend *backend);

#if defined(CUSTOM_ALLOCATOR)
int set_allocator(struct Allocator allocator);
#endif

#if defined(CUSTOM_ALLOCATOR)
void *allocate(uintptr_t size, uintptr_t align);
#endif

#if defined(CUSTOM_ALLOCATOR)
void deallocate(void *ptr, uintptr_t size, uintptr_t align);
#endif

#if defined(CUSTOM_ALLOCATOR)
void *reallocate(void *ptr, uintptr_t size, uintptr_t align, uintptr_t new_size);
#endif

struct Error *error_with_message(struct Error *err, char *context);

struct Error *error_missing_symbol(char *sym);

uintptr_t error_print(const struct Error *err, char *str, uintptr_t max_len);

void error_free(struct Error *err);

bool set_logger(struct Logger *logger);

#if defined(STD)
bool set_default_logger(void);
#endif

struct Error *os_new(struct Backend *backend, struct Os **os);

struct Error *os_new_linux(struct Backend *backend, struct Symbols *profile, struct Os **os);

void os_free(struct Os *os);

struct Error *read_virtual_memory(const struct Os *os,
                                  struct PhysicalAddress mmu_addr,
                                  struct VirtualAddress addr,
                                  uint8_t *buf,
                                  uintptr_t buf_size);

struct Error *try_read_virtual_memory(const struct Os *os,
                                      struct PhysicalAddress mmu_addr,
                                      struct VirtualAddress addr,
                                      uint8_t *buf,
                                      uintptr_t buf_size);

struct Error *read_process_memory(const struct Os *os,
                                  struct PhysicalAddress mmu_addr,
                                  struct VirtualAddress addr,
                                  struct Process proc,
                                  uint8_t *buf,
                                  uintptr_t buf_size);

struct Error *try_read_process_memory(const struct Os *os,
                                      struct PhysicalAddress mmu_addr,
                                      struct VirtualAddress addr,
                                      struct Process proc,
                                      uint8_t *buf,
                                      uintptr_t buf_size);

struct Error *os_current_process(const struct Os *os, uintptr_t vcpu, struct Process *proc);

struct Error *os_current_thread(const struct Os *os, uintptr_t vcpu, struct Thread *proc);

struct Error *os_processes(const struct Os *os, struct Process *procs, uintptr_t *n_procs);

struct Error *process_id(const struct Os *os, struct Process proc, uint64_t *pid);

struct Error *process_name(const struct Os *os, struct Process proc, char *name, uintptr_t max_len);

struct Error *process_pgd(const struct Os *os, struct Process proc, struct PhysicalAddress *pgd);

struct Error *process_path(const struct Os *os, struct Process proc, char *name, uintptr_t max_len);

struct Error *process_parent(const struct Os *os, struct Process proc, struct Process *parent);

struct Error *process_vmas(const struct Os *os,
                           struct Process proc,
                           struct Vma *vmas,
                           uintptr_t *n_vmas);

struct Error *process_threads(const struct Os *os,
                              struct Process proc,
                              struct Thread *threads,
                              uintptr_t *n_threads);

struct Error *process_children(const struct Os *os,
                               struct Process proc,
                               struct Process *children,
                               uintptr_t *n_children);

struct Error *process_modules(const struct Os *os,
                              struct Process proc,
                              struct Module *modules,
                              uintptr_t *n_modules);

struct Error *process_callstack(const struct Os *os,
                                struct Process proc,
                                struct StackFrame *frames,
                                uintptr_t *n_frames);

struct Error *thread_id(const struct Os *os, struct Thread thread, uint64_t *tid);

struct Error *thread_name(const struct Os *os, struct Thread thread, char *name, uintptr_t max_len);

struct Error *thread_process(const struct Os *os, struct Thread thread, struct Process *proc);

struct Error *vma_start(const struct Os *os, struct Vma vma, struct VirtualAddress *proc);

struct Error *vma_end(const struct Os *os, struct Vma vma, struct VirtualAddress *proc);

struct Error *vma_path(const struct Os *os, struct Vma vma, char *path, uintptr_t max_len);

struct Error *module_start(const struct Os *os,
                           struct Module module,
                           struct Process proc,
                           struct VirtualAddress *start);

struct Error *module_end(const struct Os *os,
                         struct Module module,
                         struct Process proc,
                         struct VirtualAddress *end);

struct Error *module_name(const struct Os *os,
                          struct Module module,
                          struct Process proc,
                          char *name,
                          uintptr_t max_len);

struct Error *module_path(const struct Os *os,
                          struct Module module,
                          struct Process proc,
                          char *path,
                          uintptr_t max_len);

struct Error *resolve_symbol(const struct Os *os,
                             struct Process proc,
                             struct VirtualAddress addr,
                             char *symbol,
                             uintptr_t max_len);

struct Symbols *symbols_new(void);

struct Error *symbols_load_from_bytes(struct Symbols *indexer,
                                      const char *name,
                                      const uint8_t *data,
                                      uintptr_t len);

#if defined(STD)
struct Error *symbols_load_from_file(struct Symbols *indexer, const char *path);
#endif

#if defined(STD)
struct Error *symbols_load_dir(struct Symbols *indexer, const char *path);
#endif

void symbols_free(struct Symbols *indexer);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
