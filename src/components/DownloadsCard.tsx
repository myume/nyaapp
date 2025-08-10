import { bytesToString } from "@/lib/utils";
import { DownloadInfo } from "./providers/DownloadsProvider";
import { Download, Upload } from "lucide-react";
import { Progress } from "./ui/progress";

export const DownloadsCard = ({
  download: {
    name,
    state,
    finished,
    total_bytes,
    download_speed,
    upload_speed,
    progress_bytes,
    remaining_time,
  },
}: {
  download: DownloadInfo;
}) => {
  const percentage = (progress_bytes / Math.max(total_bytes, 1)) * 100;
  return (
    <div className="p-5 border-1 rounded-xl">
      <h1>{name}</h1>
      <div className="flex items-center gap-2 py-2">
        <Progress value={percentage} />
        <h3>{percentage.toFixed(2)}%</h3>
      </div>
      <div className="flex gap-2">
        <h4>
          {bytesToString(progress_bytes)} / {bytesToString(total_bytes)}
        </h4>
        {!finished && <h4>{remaining_time} remaining</h4>}
      </div>
      <div className="flex gap-2">
        <div className="flex gap-1 items-center">
          <Download size={16} />
          <h4>{download_speed?.toFixed(2)} MiB/s</h4>
        </div>
        <div className="flex gap-1 items-center">
          <Upload size={16} />
          <h4>{upload_speed?.toFixed(2)} MiB/s</h4>
        </div>
      </div>
      <h3 className="text-xs text-muted-foreground mt-2">
        {finished ? "finished" : state === "live" ? "downloading" : state}
      </h3>
    </div>
  );
};
