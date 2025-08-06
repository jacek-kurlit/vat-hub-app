<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <v-app>
    <v-main>
      <v-container class="text-center">
        <v-row justify="center">
          <v-col cols="12">
            <h1 class="text-h3 mb-6">Welcome to Tauri + Vue + Vuetify</h1>
          </v-col>
        </v-row>

        <v-row justify="center" class="mb-6">
          <v-col cols="auto">
            <v-btn href="https://vite.dev" target="_blank" variant="text">
              <!-- <img src="/vite.svg" class="logo vite" alt="Vite logo" /> -->
            </v-btn>
          </v-col>
          <v-col cols="auto">
            <v-btn href="https://tauri.app" target="_blank" variant="text">
              <img src="./assets/tauri.svg" class="logo tauri" alt="Tauri logo" />
            </v-btn>
          </v-col>
          <v-col cols="auto">
            <v-btn href="https://vuejs.org/" target="_blank" variant="text">
              <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
            </v-btn>
          </v-col>
        </v-row>

        <v-row justify="center" class="mb-4">
          <v-col cols="12">
            <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>
          </v-col>
        </v-row>

        <v-row justify="center">
          <v-col cols="12" md="6">
            <v-form @submit.prevent="greet">
              <v-row>
                <v-col cols="8">
                  <v-text-field
                    v-model="name"
                    label="Enter a name..."
                    variant="outlined"
                    density="comfortable"
                  ></v-text-field>
                </v-col>
                <v-col cols="4">
                  <v-btn
                    type="submit"
                    color="primary"
                    size="large"
                    class="mt-1"
                  >
                    Greet
                  </v-btn>
                </v-col>
              </v-row>
            </v-form>
          </v-col>
        </v-row>

        <v-row justify="center" v-if="greetMsg">
          <v-col cols="12">
            <v-alert type="success" variant="tonal">
              {{ greetMsg }}
            </v-alert>
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<style scoped>
.logo {
  height: 4em;
  padding: 1em;
  will-change: filter;
  transition: 0.75s;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}
</style>
