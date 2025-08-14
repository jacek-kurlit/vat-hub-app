import { ref } from 'vue';

export interface Alert {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  text: string;
  timeout?: number;
}

const alerts = ref<Alert[]>([]);
let alertIdCounter = 0;

export const useAlerts = () => {
  const addAlert = (
    type: Alert['type'],
    title: string,
    text: string,
    timeout: number = 5000
  ) => {
    const id = `alert-${++alertIdCounter}`;
    const alert: Alert = {
      id,
      type,
      title,
      text,
      timeout
    };

    alerts.value.push(alert);

    // Auto-remove alert after timeout
    if (timeout > 0) {
      setTimeout(() => {
        removeAlert(id);
      }, timeout);
    }

    return id;
  };

  const removeAlert = (id: string) => {
    const index = alerts.value.findIndex(alert => alert.id === id);
    if (index > -1) {
      alerts.value.splice(index, 1);
    }
  };

  const clearAllAlerts = () => {
    alerts.value = [];
  };

  const showSuccess = (title: string, text: string, timeout?: number) => {
    return addAlert('success', title, text, timeout);
  };

  const showError = (title: string, text: string, timeout?: number) => {
    return addAlert('error', title, text, timeout);
  };

  const showWarning = (title: string, text: string, timeout?: number) => {
    return addAlert('warning', title, text, timeout);
  };

  const showInfo = (title: string, text: string, timeout?: number) => {
    return addAlert('info', title, text, timeout);
  };

  return {
    alerts,
    addAlert,
    removeAlert,
    clearAllAlerts,
    showSuccess,
    showError,
    showWarning,
    showInfo
  };
};
