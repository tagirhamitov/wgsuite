import React from 'react';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Paper from '@mui/material/Paper';
import { Button, ButtonGroup, IconButton, LinearProgress, PaletteColorOptions, createTheme, linearProgressClasses, styled } from '@mui/material';
import { Delete, SimCardDownload } from '@mui/icons-material';

const BorderLinearProgress = styled(LinearProgress)(({ theme }) => ({
  height: 10,
  borderRadius: 5,
  [`&.${linearProgressClasses.colorPrimary}`]: {
    backgroundColor: theme.palette.grey[theme.palette.mode === 'light' ? 200 : 800],
  },
  [`& .${linearProgressClasses.bar}`]: {
    borderRadius: 5,
    backgroundColor: theme.palette.mode === 'light' ? '#1a90ff' : '#308fe8',
  },
}));

const RedBorderLinearProgress = styled(LinearProgress)(({ theme }) => ({
  height: 10,
  borderRadius: 5,
  [`&.${linearProgressClasses.colorPrimary}`]: {
    backgroundColor: theme.palette.grey[theme.palette.mode === 'light' ? 200 : 800],
  },
  [`& .${linearProgressClasses.bar}`]: {
    borderRadius: 5,
    backgroundColor: theme.palette.mode === 'light' ? '#E83030' : '#E83030',
  },
}));

const makeProgressBar = (bytes: number, maxBytes: number) => {
  const percentage = Math.ceil(bytes / (maxBytes * 1024 * 1024 * 1024) * 100);
  if (percentage >= 100) {
    return <RedBorderLinearProgress variant="determinate" value={100} />;
  }
  return <BorderLinearProgress variant="determinate" value={percentage} />;
};

interface Client {
  id: number;
  name: string;
  ip: string;
  last_connected: number;
  uploaded: number;
  downloaded: number;
}

interface ClientsTableProps {
  maxBytes: string;
};

function bytesToString(bytes: number): string {
  if (bytes < 1024) {
    return bytes + " B";
  }
  bytes /= 1024;

  if (bytes < 1024) {
    return bytes.toFixed(2) + " KB";
  }
  bytes /= 1024;

  if (bytes < 1024) {
    return bytes.toFixed(2) + " MB";
  }
  bytes /= 1024;

  return bytes.toFixed(2) + " GB";
}

function secondsToString(seconds: number): string {
  if (seconds < 60) {
    return seconds + "s";
  }

  let minutes = Math.floor(seconds / 60);
  seconds = seconds % 60;
  if (minutes < 60) {
    return minutes + "m " + seconds + "s";
  }

  let hours = Math.floor(minutes / 60);
  minutes = minutes % 60;
  if (hours < 24) {
    return hours + "h " + minutes + "m";
  }

  let days = Math.floor(hours / 24);
  hours = hours % 24;
  return days + "d " + hours + "h";
}

function ClientsTable(props: ClientsTableProps) {
  const [clients, setClients] = React.useState<Client[]>([]);
  React.useEffect(() => {
    setInterval(() => {
      fetch("http://localhost:3000/clients").then((resp) => {
        return resp.json();
      }).then(setClients);
    }, 1000);
  }, []);

  return (
    <TableContainer component={Paper}>
      <Table sx={{ minWidth: 650 }} aria-label="simple table">
        <TableHead style={{}}>
          <TableRow>
            <TableCell>Name</TableCell>
            <TableCell>IP address</TableCell>
            <TableCell>Connected</TableCell>
            <TableCell>Uploaded</TableCell>
            <TableCell>Downloaded</TableCell>
            <TableCell align='right'>Manage</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {clients.map((item) => {
            const uploadedBytesStr = bytesToString(item.uploaded);
            const downloadedBytesStr = bytesToString(item.downloaded);

            return (
              <TableRow>
                <TableCell>{item.name}</TableCell>
                <TableCell>{item.ip}</TableCell>
                <TableCell>{secondsToString(item.last_connected)} ago</TableCell>
                <TableCell>{uploadedBytesStr} / {props.maxBytes}GB{makeProgressBar(item.uploaded, +props.maxBytes)}</TableCell>
                <TableCell>{downloadedBytesStr} / {props.maxBytes}GB{makeProgressBar(item.downloaded, +props.maxBytes)}</TableCell>
                <TableCell align='right'>
                  <ButtonGroup size="small" variant="contained">
                    <Button href={"http://localhost:3000/config/" + item.id}><SimCardDownload></SimCardDownload></Button>
                    <Button onClick={() => {
                      fetch("http://localhost:3000/clients/" + item.id, {
                        method: "delete",
                      });
                    }}><Delete></Delete></Button>
                  </ButtonGroup>
                </TableCell>
              </TableRow>
            );
          })}
        </TableBody>
      </Table>
    </TableContainer>
  );
}

export default ClientsTable;
