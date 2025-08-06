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
        <template v-slot:item.vatPaymentStatus="{ item }">
          <v-chip
            :color="getStatusColor(item.vatPaymentStatus)"
            :prepend-icon="getStatusIcon(item.vatPaymentStatus)"
            size="small"
          >
            {{ getStatusText(item.vatPaymentStatus) }}
          </v-chip>
        </template>

        <template v-slot:item.lastUpdateDate="{ item }">
          {{ formatDate(item.lastUpdateDate) }}
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

enum VatPaymentStatus {
  ACTIVE = 'active',
  EXEMPT = 'exempt',
  INACTIVE = 'inactive',
  UNKNOWN = 'unknown'
}

interface Contractor {
  id: string;
  name: string;
  vatId: string;
  vatPaymentStatus: VatPaymentStatus;
  lastUpdateDate: string;
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
    key: 'vatId',
    sortable: true,
  },
  {
    title: 'Status VAT',
    key: 'vatPaymentStatus',
    sortable: true,
    align: 'center' as const,
  },
  {
    title: 'Ostatnia aktualizacja',
    key: 'lastUpdateDate',
    sortable: true,
  },
];

// Mock data function - replace with actual Tauri API call later
const fetchContractors = async (page: number, limit: number, search?: string): Promise<{ data: Contractor[], total: number }> => {
  // Simulate API delay
  await new Promise(resolve => setTimeout(resolve, 500));
  
  // Mock data
  const mockData: Contractor[] = [
    {
      id: '1',
      name: 'ABC Sp. z o.o.',
      vatId: '1234567890',
      vatPaymentStatus: VatPaymentStatus.ACTIVE,
      lastUpdateDate: '2024-01-15T10:30:00Z',
    },
    {
      id: '2',
      name: 'XYZ S.A.',
      vatId: '0987654321',
      vatPaymentStatus: VatPaymentStatus.INACTIVE,
      lastUpdateDate: '2024-01-14T14:20:00Z',
    },
    {
      id: '3',
      name: 'DEF Przedsiębiorstwo',
      vatId: '1122334455',
      vatPaymentStatus: VatPaymentStatus.UNKNOWN,
      lastUpdateDate: '2024-01-13T09:15:00Z',
    },
    {
      id: '4',
      name: 'GHI Handel',
      vatId: '5566778899',
      vatPaymentStatus: VatPaymentStatus.ACTIVE,
      lastUpdateDate: '2024-01-12T16:45:00Z',
    },
    {
      id: '5',
      name: 'JKL Usługi',
      vatId: '9988776655',
      vatPaymentStatus: VatPaymentStatus.EXEMPT,
      lastUpdateDate: '2024-01-11T11:30:00Z',
    },
  ];

  // Filter data based on search query
  let filteredData = mockData;
  if (search && search.trim()) {
    const searchLower = search.toLowerCase().trim();
    filteredData = mockData.filter(contractor => 
      contractor.name.toLowerCase().includes(searchLower) ||
      contractor.vatId.includes(searchLower)
    );
  }

  const startIndex = (page - 1) * limit;
  const endIndex = startIndex + limit;
  const paginatedData = filteredData.slice(startIndex, endIndex);

  return {
    data: paginatedData,
    total: filteredData.length,
  };
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

const getStatusColor = (status: VatPaymentStatus): string => {
  switch (status) {
    case VatPaymentStatus.ACTIVE:
      return 'success';
    case VatPaymentStatus.EXEMPT:
      return 'info';
    case VatPaymentStatus.INACTIVE:
      return 'error';
    case VatPaymentStatus.UNKNOWN:
    default:
      return 'warning';
  }
};

const getStatusIcon = (status: VatPaymentStatus): string => {
  switch (status) {
    case VatPaymentStatus.ACTIVE:
      return 'mdi-check-circle';
    case VatPaymentStatus.EXEMPT:
      return 'mdi-shield-check';
    case VatPaymentStatus.INACTIVE:
      return 'mdi-close-circle';
    case VatPaymentStatus.UNKNOWN:
    default:
      return 'mdi-help-circle';
  }
};

const getStatusText = (status: VatPaymentStatus): string => {
  switch (status) {
    case VatPaymentStatus.ACTIVE:
      return 'Aktywny';
    case VatPaymentStatus.EXEMPT:
      return 'Zwolniony';
    case VatPaymentStatus.INACTIVE:
      return 'Nieaktywny';
    case VatPaymentStatus.UNKNOWN:
    default:
      return 'Nieznany';
  }
};

const formatDate = (dateString: string): string => {
  const date = new Date(dateString);
  return date.toLocaleDateString('pl-PL', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  });
};

onMounted(() => {
  loadContractors();
});
</script>

<style scoped>
/* Contractors styles */
</style>
