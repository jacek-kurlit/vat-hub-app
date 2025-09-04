<template>
  <div>
    <h1 class="text-h4 mb-4">Kontrahenci</h1>
    
    <v-card>
      <v-card-title>
        <v-row align="center">
          <v-col>
            Lista kontrahentów
          </v-col>
          <v-col cols="auto">
            <v-btn 
              color="primary" 
              @click="refreshData"
              :loading="loading"
            >
              <v-icon left>mdi-refresh</v-icon>
              Odśwież
            </v-btn>
          </v-col>
        </v-row>
      </v-card-title>

      <v-card-text>
        <v-row>
          <v-col cols="12" md="6">
            <v-text-field
              v-model="searchQuery"
              label="Szukaj po nazwie lub NIP"
              prepend-inner-icon="mdi-magnify"
              clearable
              variant="outlined"
              density="compact"
              @input="handleSearch"
            ></v-text-field>
          </v-col>
        </v-row>
      </v-card-text>

      <v-data-table
        :headers="headers"
        :items="contractors"
        :loading="loading"
        :items-per-page="itemsPerPage"
        :items-per-page-options="[10, 20, 30]"
        :page="currentPage"
        :server-items-length="totalItems"
        @update:page="handlePageChange"
        @update:items-per-page="handleItemsPerPageChange"
        class="elevation-1"
      >
        <template v-slot:item.vat_status="{ item }">
          <v-chip
            :color="getStatusColor(item.vat_status)"
            :prepend-icon="getStatusIcon(item.vat_status)"
            size="small"
          >
            {{item.vat_status }}
          </v-chip>
        </template>

        <template v-slot:no-data>
          <v-alert type="info" class="ma-4">
            Brak danych do wyświetlenia
          </v-alert>
        </template>
      </v-data-table>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';


interface Contractor {
  name: string;
  nip: string;
  vat_status: string;
  regon: string;
  krs: string;
  residence_address?: string;
  working_address?: string;
  accounts_numbers: string[];
}

const loading = ref(false);
const contractors = ref<Contractor[]>([]);
const currentPage = ref(1);
const itemsPerPage = ref(10);
const totalItems = ref(0);
const searchQuery = ref('');

const headers = [
  {
    title: 'Nazwa',
    key: 'name',
    sortable: true,
  },
  {
    title: 'NIP',
    key: 'nip',
    sortable: true,
  },
  {
    title: 'Status',
    key: 'vat_status',
    sortable: true,
    align: 'center' as const,
  },
  {
    title: 'REGON',
    key: 'regon',
    sortable: true,
  },
  {
    title: 'KRS',
    key: 'krs',
    sortable: true,
  },
];

const fetchContractors = async (page: number, limit: number, search?: string): Promise<{ data: Contractor[], total: number }> => {
  try {
    const contractors = await invoke<Contractor[]>('fetch_contractors', { 
      page: page,
      pageSize: limit ,
      search: search ? search.trim() : null,
    });
    
    // Filter data based on search query on frontend side
    let filteredData = contractors;
    if (search && search.trim()) {
      const searchLower = search.toLowerCase().trim();
      filteredData = contractors.filter(contractor => 
        contractor.name.toLowerCase().includes(searchLower) ||
        contractor.nip.includes(searchLower)
      );
    }

    return {
      data: filteredData,
      total: filteredData.length,
    };
  } catch (error) {
    console.error('Error fetching contractors from Rust:', error);
    throw error;
  }
};

const loadContractors = async () => {
  loading.value = true;
  try {
    const result = await fetchContractors(currentPage.value, itemsPerPage.value, searchQuery.value);
    contractors.value = result.data;
    totalItems.value = result.total;
  } catch (error) {
    console.error('Error loading contractors:', error);
  } finally {
    loading.value = false;
  }
};

const refreshData = () => {
  loadContractors();
};

const handlePageChange = (page: number) => {
  currentPage.value = page;
  loadContractors();
};

const handleItemsPerPageChange = (newSize: number) => {
  itemsPerPage.value = newSize;
  currentPage.value = 1;
  loadContractors();
};

const handleSearch = () => {
  currentPage.value = 1;
  loadContractors();
};

const getStatusColor = (status: string): string => {
  switch (status.toLowerCase()) {
    case 'czynny':
      return 'success';
    case 'zwolniony':
      return 'info';
    case 'nieczynny':
      return 'error';
    default:
      return 'warning';
  }
};

const getStatusIcon = (status: string): string => {
  switch (status.toLowerCase()) {
    case 'czynny':
      return 'mdi-check-circle';
    case 'zwolniony':
      return 'mdi-shield-check';
    case 'nieczynny':
      return 'mdi-close-circle';
    default:
      return 'mdi-help-circle';
  }
};

onMounted(() => {
  loadContractors();
});
</script>

<style scoped>
/* Contractors styles */
</style>
