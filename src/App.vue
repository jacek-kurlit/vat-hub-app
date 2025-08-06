<script setup lang="ts">
import { ref } from "vue";

const selectedItem = ref("notyfikacje");

const menuItems = [
  {
    title: "Notyfikacje",
    value: "notyfikacje",
    icon: "mdi-bell"
  },
  {
    title: "Kontrahenci",
    value: "kontrahenci",
    icon: "mdi-account-group"
  }
];

const selectMenuItem = (item: string) => {
  selectedItem.value = item;
};
</script>

<template>
  <v-app>
    <v-navigation-drawer permanent width="200">
      <v-list>
        <v-list-item
          v-for="item in menuItems"
          :key="item.value"
          :prepend-icon="item.icon"
          :title="item.title"
          :active="selectedItem === item.value"
          :color="selectedItem === item.value ? 'green' : undefined"
          @click="selectMenuItem(item.value)"
          class="mb-1"
        ></v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <v-container fluid>
        <v-row>
          <v-col cols="12">
            <div v-if="selectedItem === 'notyfikacje'">
              <h1 class="text-h4 mb-4">Notyfikacje</h1>
              <v-card>
                <v-card-text>
                  <p>Tutaj będą wyświetlane notyfikacje systemu.</p>
                  <p>Placeholder dla funkcjonalności notyfikacji.</p>
                </v-card-text>
              </v-card>
            </div>
            
            <div v-if="selectedItem === 'kontrahenci'">
              <h1 class="text-h4 mb-4">Kontrahenci</h1>
              <v-card>
                <v-card-text>
                  <p>Tutaj będzie lista kontrahentów.</p>
                  <p>Placeholder dla zarządzania kontrahentami.</p>
                </v-card-text>
              </v-card>
            </div>
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<style scoped>
/* Admin panel styles */
</style>
