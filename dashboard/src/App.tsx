import React from 'react';
import './App.css';
import AddUserForm from './components/AddUserForm';
import ClientsTable from './components/ClientsTable';
import ProgressBarMax from './components/ProgressBarMax';
import { Container } from '@mui/material';

function App() {
  const [maxBytes, setMaxBytes] = React.useState<string>("100");
  return (
    <Container>
      <ProgressBarMax value={maxBytes} onValueChange={setMaxBytes}></ProgressBarMax>
      <AddUserForm></AddUserForm>
      <ClientsTable maxBytes={maxBytes}></ClientsTable>
    </Container>
  );
}

export default App;
