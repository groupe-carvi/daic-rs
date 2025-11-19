#pragma once

// Simple test header to verify autocxx works without needing full depthai-core
namespace test {
    inline int add(int a, int b) {
        return a + b;
    }
    
    inline const char* get_version() {
        return "0.1.0-test";
    }
}
