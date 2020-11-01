//
// Created by avery on 27/10/2020.
//


#include "ProtonCaller.h"
#include "ProtonArguments.h"
#include "ProtonSetup.h"
#include <cstdio>
#include <cstdlib>


struct argsStruct {
    std::string _argv1;
    std::string _argv2;
    std::string _argv3;
};
struct argsStruct proArgs;

void Args(ProtonCaller &proObj, int argc, char *argv[]) {
    if (argc == 1) {
        std::cout << "You must supply argument. View help (-h).\n";
        exit(EXIT_FAILURE);
    }
    argsMain(argc, argv);
    defineArgs1(proObj, argc, argv);
    defineArgs2(argc, argv);
}

void argsMain(int argc, char *argv[]) {
    if (argv[1] != nullptr) {
        proArgs._argv1 = argv[1];
    } else if (argv[2] != nullptr) {
        proArgs._argv2 = argv[2];
    } else if (argv[3] != nullptr) {
        proArgs._argv3 = argv[3];
    } else {
        std::cout << "Crashed.\n";
        exit(EXIT_FAILURE);
    }
}

void defineArgs1(ProtonCaller &proObj, int argc, char *argv[]) {
    if (proArgs._argv1 == "-h") {
        helpMsg();
        exit(EXIT_SUCCESS);
    } else if (proArgs._argv1 == "-v") {
        PRVersion();
        exit(EXIT_SUCCESS);
    } else if (proArgs._argv1 == "-c") {
        std::cout << "Custom mode: will not check for Proton.\n";
        if (argv[3] != nullptr) {
            proObj.custom = true;
            proArgs._argv3 = argv[3];
            return;
        }
    } else if (proArgs._argv1 == "--setup") {
        setup("--setup");
    }
}

void defineArgs2(int argc, char *argv[]) {
    if (argv[2] != nullptr) {
        proArgs._argv2 = argv[2];
    } else {
        std::cout << "What program?\n";
        exit(EXIT_FAILURE);
    }
}

void setEnvironment(ProtonCaller &proObj) {
    if (proObj.custom) {
        proObj.program = proArgs._argv3;
        proObj.proton_path = proArgs._argv2;
        findEnv(1);
    } else {
        proObj.common = findEnv(0);
        if (proArgs._argv1 == "5") {proObj.proton = "5.0";}
        else {proObj.proton = proArgs._argv1;}
        proObj.program = proArgs._argv2;
        std::string _proton = "Proton ";
        proObj.proton_path = proObj.common + _proton;
    }
}

const char* findEnv(int rtn) {
    if (getenv(STEAM) != nullptr) {
        std::cout << STEAM << " located at: " << getenv(STEAM) <<"\n";
    } else {
        std::cout << STEAM << " must be added to your environment. Proton Will not run without it.\n";
        exit(EXIT_FAILURE);
    }
    if (rtn == 1){return nullptr;}
    if (getenv(COMMON) != nullptr) {
        const char *cCommon = getenv(COMMON);
        std::cout << COMMON << " located at: " << cCommon << "\n";
        return cCommon;
    } else {
        setup(COMMON);
        exit(EXIT_FAILURE);
    }
}

void helpMsg() {
    FILE *fFile;
    fFile = fopen("/usr/share/proton-caller/HELP", "r");
    if (fFile == nullptr) {
        std::cout << "Error opening help message.\n";
        exit(EXIT_FAILURE);
    }
    char c;
    c = fgetc(fFile);
    while (c != EOF) {
        std::cout << c;
        c = fgetc(fFile);
    }
    fclose(fFile);
}

