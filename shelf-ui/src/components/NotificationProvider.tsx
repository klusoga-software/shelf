import React, { createContext, ReactNode, useEffect, useState } from "react";
import { Alert, AlertProps } from "@cloudscape-design/components";

interface NotificationProviderProps {
  children: ReactNode;
}

interface NotificationProviderContextProps {
  showNotification: (notification: Notification) => void;
  alerts: () => React.ReactNode;
}

export const NotificationContext = createContext<
  NotificationProviderContextProps | undefined
>(undefined);

export interface Notification {
  type: AlertProps.Type;
  header: string;
  message: string;
}

export const NotificationProvider: React.FC<NotificationProviderProps> = ({
  children,
}) => {
  const [notification, setNotification] = useState<Notification | undefined>();

  const showNotification = (notification: Notification): void => {
    setNotification(notification);
  };

  useEffect(() => {
    const timer = setTimeout(() => {
      setNotification(undefined);
    }, 3000);

    return () => clearTimeout(timer);
  }, [notification]);

  const alerts = () => {
    if (notification) {
      return (
        <Alert
          dismissible={true}
          onDismiss={() => {
            setNotification(undefined);
          }}
          type={notification?.type}
          header={notification?.header}
        >
          {notification?.message}
        </Alert>
      );
    }
  };

  return (
    <NotificationContext.Provider value={{ showNotification, alerts }}>
      {children}
    </NotificationContext.Provider>
  );
};
