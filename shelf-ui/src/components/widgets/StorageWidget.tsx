import { Link } from "@cloudscape-design/components";
import { useNavigate } from "react-router-dom";

const StorageWidget = ({ count, link }: { count: number; link: string }) => {
  const navigate = useNavigate();

  function formatBytes(bytes: number, decimals: number = 2): string {
    if (bytes === 0) return "0 Bytes";

    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + " " + sizes[i];
  }

  return (
    <Link
      onClick={() => {
        navigate(link);
      }}
      variant="awsui-value-large"
    >
      {formatBytes(count)}
    </Link>
  );
};

export default StorageWidget;
