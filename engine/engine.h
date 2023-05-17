//
// Created by zhangyifan on 2023/5/17.
//

#include "cxx.h"
#include <memory>

#ifndef IMLIBC_ENGINE_H
#define IMLIBC_ENGINE_H

#define operatorINVOKE operator()

namespace cn {
    namespace rongcloud {
        using MyFunction = std::function<void()>;
    }
};



#endif //IMLIBC_ENGINE_H
