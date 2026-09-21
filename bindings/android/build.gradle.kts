// Android library module producing an AAR for `implementation("com.zvvnmod:mongol-convert-android:<ver>")`.
// The release CI fills in the two generated inputs before building:
//   - src/main/kotlin/uniffi/mongol_convert_uniffi/mongol_convert_uniffi.kt   (uniffi-bindgen --language kotlin)
//   - src/main/jniLibs/<abi>/libmongol_convert_uniffi.so            (cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 ...)
plugins {
    id("com.android.library") version "8.5.0"
    id("org.jetbrains.kotlin.android") version "2.0.0"
    `maven-publish`
}

group = "com.zvvnmod"
version = "0.7.0"

android {
    namespace = "com.zvvnmod.mongolconvert"
    compileSdk = 34
    defaultConfig {
        minSdk = 21
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    sourceSets["main"].kotlin.srcDir("src/main/kotlin")
    sourceSets["main"].jniLibs.srcDir("src/main/jniLibs")
}

// Align Java + Kotlin JVM targets (Gradle/Kotlin now error on a mismatch).
kotlin {
    jvmToolchain(17)
}

dependencies {
    // UniFFI's Kotlin runtime dependency.
    implementation("net.java.dev.jna:jna:5.14.0@aar")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.8.1")
}

publishing {
    publications {
        register<MavenPublication>("release") {
            afterEvaluate { from(components["release"]) }
        }
    }
    // For Maven Central / GitHub Packages add the repository + signing here, or publish via JitPack.
}
